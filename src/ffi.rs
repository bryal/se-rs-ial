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

#![allow(non_snake_case, non_camel_case_types, dead_code)]

use libc::c_char;
use winapi::{ DWORD, HANDLE, BOOL, WORD, BYTE };

#[repr(C)]
pub struct i2 {
	lo: bool,
	hi: bool,
}

#[repr(C)]
pub struct i17 {
	a: i16,
	b: bool,
}

#[repr(C)]
pub struct DCB {
	pub DCBlength: DWORD,
	pub BaudRate: DWORD,
	pub fBinary: bool,
	pub fParity: bool,
	pub fOutxCtsFlow: bool,
	pub fOutxDsrFlow: bool,
	pub fDtrControl: i2,
	pub fDsrSensitivity: bool,
	pub fTXContinueOnXoff: bool,
	pub fOutX: bool,
	pub fInX: bool,
	pub fErrorChar: bool,
	pub fNull: bool,
	pub fRtsControl: i2,
	pub fAbortOnError: bool,
	pub fDummy2: i17,
	pub wReserved: WORD,
	pub XonLim: WORD,
	pub XoffLim: WORD,
	pub ByteSize: BYTE,
	pub Parity: BYTE,
	pub StopBits: BYTE,
	pub XonChar: c_char,
	pub XoffChar: c_char,
	pub ErrorChar: c_char,
	pub EofChar: c_char,
	pub EvtChar: c_char,
	pub wReserved1: WORD,
}

#[link(name = "kernel32")]
extern "system" {
	pub fn GetComState(file_handle: HANDLE, dcb: *mut DCB) -> BOOL;
	pub fn SetCommState(file_handle: HANDLE, dcb: *mut DCB) -> BOOL;
}