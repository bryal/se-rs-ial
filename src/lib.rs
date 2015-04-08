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
use winapi::HANDLE;
use std::{ ptr, mem };

mod ffi;

pub struct Connection {
	// Pointer to the serial connection
	com_handle: HANDLE
}

impl Connection {
	pub fn new(port: &str, baud_rate: u32) -> Result<Connection, &'static str> {
		let (com_handle, cf_result) = unsafe {
			let mut port_u16: Vec<_> = port.utf16_units().collect();
			port_u16.push(0);
			(
				kernel32::CreateFileW(port_u16.as_ptr(),
					winapi::GENERIC_READ | winapi::GENERIC_WRITE,
					0,
					ptr::null_mut(),
					winapi::OPEN_EXISTING,
					winapi::FILE_FLAG_OVERLAPPED,
					ptr::null_mut()),
				kernel32::GetLastError()
			)
		};

		if com_handle == winapi::INVALID_HANDLE_VALUE {
			match cf_result {
				winapi::ERROR_ACCESS_DENIED => Err("Access denied, port might be busy"),
				winapi::ERROR_FILE_NOT_FOUND => Err("COM port does not exist"),
				_ => Err("Invalid COM port handle")
			}
		} else {
			let mut conn = Connection{ com_handle: com_handle };
			match conn.set_baud_rate(baud_rate) {
				Ok(_) => Ok(conn),
				Err(_) => Err("Error setting baud rate"),
			}
		}
	}

	pub fn set_baud_rate(&mut self, baud_rate: u32) -> Result<(), ()> {
		unsafe {
			let mut dcb = mem::zeroed();
			if GetCommState(self.com_handle, &mut dcb) == 0 {
				Err(())
			} else {
				dcb.BaudRate = baud_rate;
				if SetCommState(self.com_handle, &mut dcb) == 0 {
					Err(())
				} else {
					Ok(())
				}
			}
		}
	}
}

#[test]
fn test() {
	let con = Connection::new("COM8", 9600).unwrap();
}