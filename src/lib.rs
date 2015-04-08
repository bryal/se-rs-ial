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

#![feature(collections, step_by)]

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
impl Drop for Connection {
	fn drop(&mut self) {
		let e = unsafe { kernel32::CloseHandle(self.comm_handle) };
		if e < 1 {
			panic!("CloseHandle failed with 0x{:x}", e)
		}
	}
}
unsafe impl Send for Connection { }

// Colorswirl. Works with arduino running LEDstream
#[test]
fn colorswirl_test() {
	use std::thread;
	use std::io::Write;

	let port = "COM8";
	let baud_rate = 115_200;
	let mut connection = Connection::new(port, baud_rate).unwrap();

	let n_leds = 32;
	let pixel_size = 3;
	let header_size = 6;
	let mut buffer: Vec<u8> = (0..(header_size + n_leds * pixel_size)).map(|_| 0).collect();

	// A special header / magic word is expected by the corresponding LED streaming code 
	// running on the Arduino. This only needs to be initialized once because the number of  
	// LEDs remains constant:
	buffer[0] = 'A' as u8;                    // Magic word
	buffer[1] = 'd' as u8;
	buffer[2] = 'a' as u8;
	buffer[3] = ((n_leds - 1) >> 8) as u8;    // LED count high byte
	buffer[4] = ((n_leds - 1) & 0xff) as u8;  // LED count low byte
	buffer[5] = buffer[3] ^ buffer[4] ^ 0x55; // Checksum
	
	let mut main_sin = 0.0_f32;
	let mut main_hue = 0_u16;

	for _ in 0..1_000 {
		let mut internal_sin = main_sin;
		let mut internal_hue = main_hue;

		let (mut r, mut g, mut b): (u8, u8, u8);
		// Start at position 6, after the LED header/magic word
		for i in (6..buffer.len()).step_by(3) {
			// Fixed-point hue-to-RGB conversion.  'internal_hue' is an integer in the
			// range of 0 to 1535, where 0 = red, 256 = yellow, 512 = green, etc.
			// The high byte (0-5) corresponds to the sextant within the color
			// wheel, while the low byte (0-255) is the fractional part between
			// the primary/secondary colors.
			let pri_sec_frac = (internal_hue & 255) as u8;
			match (internal_hue >> 8) % 6 {
				0 => {
					r = 255;
					g = pri_sec_frac;
					b = 0;
				}, 1 => {
					r = 255 - pri_sec_frac;
					g = 255;
					b = 0;
				}, 2 => {
					r = 0;
					g = 255;
					b = pri_sec_frac;
				}, 3 => {
					r = 0;
					g = 255 - pri_sec_frac;
					b = 255;
				}, 4 => {
					r = pri_sec_frac;
					g = 0;
					b = 255;
				}, _ => {
					r = 255;
					g = 0;
					b = 255 - pri_sec_frac;
				}
			}

			// Resulting hue is multiplied by brightness in the range of 0 to 255
			// (0 = off, 255 = brightest).  Gamma corrrection (the 'powf' function
			// here) adjusts the brightness to be more perceptually linear.
			let brightness = (0.5 + internal_sin.sin() * 0.5).powf(2.8);
			buffer[i]     = (r as f32 * brightness) as u8;
			buffer[i + 1] = (g as f32 * brightness) as u8;
			buffer[i + 2] = (b as f32 * brightness) as u8;

			// Each pixel is slightly offset in both hue and brightness
			internal_hue += 40;
			internal_sin += 0.3;
		}

		// Slowly rotate hue and brightness in opposite directions
		main_hue = (main_hue + 4) % 1536;
		main_sin -= 0.03;

		// Issue color data to LEDs
		connection.write(&buffer[..]).unwrap();

		// The arduino can't handle it if we go too fast
		thread::sleep_ms(2);
	}
}