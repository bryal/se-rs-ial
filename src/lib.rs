// The MIT License (MIT)
//
// Copyright (c) 2015 Johan Johansson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#![feature(collections)]

extern crate libc;
extern crate winapi;

pub use ffi::*;

use libc::funcs::extra::kernel32;
use libc::c_void;
use winapi::HANDLE;
use std::{ ptr, mem, io };
use std::io::{Error, ErrorKind};

mod ffi;

pub struct Connection {
	// Pointer to the serial connection
	comm_handle: HANDLE
}

impl Connection {
	pub fn new(port: &str, baud_rate: u32) -> io::Result<Connection> {
		let (comm_handle, cf_result) = unsafe {
			let mut port_u16: Vec<_> = port.utf16_units().collect();
			port_u16.push(0);
			(
				kernel32::CreateFileW(port_u16.as_ptr(),
					winapi::GENERIC_READ | winapi::GENERIC_WRITE,
					0,
					ptr::null_mut(),
					winapi::OPEN_EXISTING,
					0,
					ptr::null_mut()),
				kernel32::GetLastError()
			)
		};

		if comm_handle == winapi::INVALID_HANDLE_VALUE {
			Err(match cf_result {
				winapi::ERROR_ACCESS_DENIED =>
					Error::new(ErrorKind::AlreadyExists, "Access denied, port might be busy"),
				winapi::ERROR_FILE_NOT_FOUND =>
					Error::new(ErrorKind::NotFound, "COM port does not exist"),
				_ => Error::new(ErrorKind::Other, "Invalid COM port handle")
			})
		} else {
			let mut conn = Connection{ comm_handle: comm_handle };
			match conn.set_baud_rate(baud_rate) {
				Ok(_) => Ok(conn),
				Err(_) => Err(Error::new(ErrorKind::Other, "Error setting baud rate")),
			}
		}
	}

	pub fn set_baud_rate(&mut self, baud_rate: u32) -> Result<(), ()> {
		unsafe {
			let mut dcb = mem::zeroed();
			if GetCommState(self.comm_handle, &mut dcb) <= 0 {
				Err(())
			} else {
				dcb.BaudRate = baud_rate;
				if SetCommState(self.comm_handle, &mut dcb) <= 0 {
					Err(())
				} else {
					Ok(())
				}
			}
		}
	}
}
impl io::Read for Connection {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		let mut n_bytes_read = 0;

		let (succeded, err) = unsafe { (
			kernel32::ReadFile(self.comm_handle,
				buf.as_mut_ptr() as *mut c_void,
				buf.len() as u32,
				&mut n_bytes_read,
				ptr::null_mut()) > 0,
			kernel32::GetLastError()
		) };

		if succeded {
			Ok(n_bytes_read as usize)
		} else {
			Err(match err {
				winapi::ERROR_INVALID_USER_BUFFER =>
					Error::new(ErrorKind::InvalidInput, "Supplied buffer is invalid"),
				winapi::ERROR_NOT_ENOUGH_MEMORY =>
					Error::new(ErrorKind::Other, "Too many I/O requests"),
				winapi::ERROR_OPERATION_ABORTED =>
					Error::new(ErrorKind::Interrupted, "Operation was canceled"),
				_ => Error::new(ErrorKind::Other, format!("Read failed with 0x{:x}", err))
			})
		}
	}
}
impl io::Write for Connection {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let mut n_bytes_written = 0;

		let (succeded, err) = unsafe { (
			kernel32::WriteFile(self.comm_handle,
				mem::transmute(buf.as_ptr()),
				buf.len() as u32,
				&mut n_bytes_written,
				ptr::null_mut()) > 0,
			kernel32::GetLastError()
		) };

		if succeded {
			Ok(n_bytes_written as usize)
		} else {
			Err(match err {
				winapi::ERROR_INVALID_USER_BUFFER =>
					Error::new(ErrorKind::InvalidInput, "Supplied buffer is invalid"),
				winapi::ERROR_NOT_ENOUGH_MEMORY =>
					Error::new(ErrorKind::Other, "Too many I/O requests"),
				winapi::ERROR_OPERATION_ABORTED =>
					Error::new(ErrorKind::Interrupted, "Operation was canceled"),
				_ => Error::new(ErrorKind::Other, format!("Write failed with 0x{:x}", err))
			})
		}
	}

	fn flush(&mut self) -> io::Result<()> {
		Ok(())
	}
}

#[test]
fn test() {
	let con = Connection::new("COM8", 9600).unwrap();
}