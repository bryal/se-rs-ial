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

extern crate kernel32;
extern crate winapi;

use winapi::HANDLE;
use std::ffi::CString;
use std::ptr;

pub struct Connection {
	// Pointer to the serial connection
	com_handle: HANDLE
}

impl Connection {
	pub fn new<T: Into<Vec<u8>>>(port: T, baud_rate: u32) -> Result<Connection, &'static str> {
		let com_handle = unsafe {
			kernel32::CreateFileA(CString::new(port).unwrap().as_ptr(),
				winapi::GENERIC_READ | winapi::GENERIC_WRITE,
				0,
				ptr::null_mut(),
				winapi::OPEN_EXISTING,
				winapi::FILE_FLAG_OVERLAPPED,
				ptr::null_mut())
		};

		if com_handle == winapi::INVALID_HANDLE_VALUE {
			Err("Invalid COM port handle. Port might be in busy")
		} else {
			Ok(Connection{ com_handle: com_handle })
		}
	}
}

#[test]
fn test() {
	let com_handle = unsafe { kernel32::CreateFileA(CString::new("COM8").unwrap().as_ptr(),
		winapi::GENERIC_READ | winapi::GENERIC_WRITE,
		0,
		ptr::null_mut(),
		winapi::OPEN_EXISTING,
		winapi::FILE_FLAG_OVERLAPPED,
		ptr::null_mut()) };

	assert!(com_handle != winapi::INVALID_HANDLE_VALUE);
}