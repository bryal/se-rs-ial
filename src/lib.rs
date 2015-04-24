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

#[macro_use] extern crate bitflags;
extern crate libc;
#[cfg(windows)]
extern crate serial_win;
#[cfg(unix)]
extern crate serial;

use std::io::{ self, Error, ErrorKind };

#[cfg(windows)]
pub struct Connection {
	conn: serial_win::Connection
}
#[cfg(windows)]
impl Connection {
	pub fn open(port: &str, baud_rate: u32) -> io::Result<Connection> {
		serial_win::Connection::new(port, baud_rate).map(|conn| Connection { conn: conn })
	}
}

#[cfg(unix)]
pub struct Connection {
	conn: serial::SerialPort
}
#[cfg(unix)]
impl Connection {
	/// Open a connection through a serial port.
	///
	/// Supported baud rates are:
	/// 0, 50, 75, 110, 134, 150, 200, 300, 600, 1200, 1800,
	/// 2400, 4800, 9600, 19200, 38400, 57600, 115200, 230400
	pub fn open(port: &str, baud_rate: u32) -> io::Result<Connection> {
		use serial::BaudRate::*;
		use std::path::Path;

		let baud_rate = match baud_rate {
			0 => B0,
			50 => B50,
			75 => B75,
			110 => B110,
			134 => B134,
			150 => B150,
			200 => B200,
			300 => B300,
			600 => B600,
			1200 => B1200,
			1800 => B1800,
			2400 => B2400,
			4800 => B4800,
			9600 => B9600,
			19200 => B19200,
			38400 => B38400,
			57600 => B57600,
			115200 => B115200,
			230400 => B230400,
			_ => return Err(Error::new(ErrorKind::InvalidInput, "Unsupported baud rate. View docs for more info"))
		};

		match serial::SerialPort::open(&Path::new(port)) {
			Ok(mut conn) => match conn.set_baud_rate(serial::Direction::Both, baud_rate) {
				Ok(_) => Ok(Connection{ conn: conn }),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(unix)]
	const PORTS: [&'static str; 10] = [
		"/dev/ttyS0", "/dev/ttyS1", "/dev/ttyS2", "/dev/ttyS3", "/dev/ttyS4",
		"/dev/ttyUSB0", "/dev/ttyUSB1", "/dev/ttyUSB2",
		"/dev/ttyACM0", "/dev/ttyACM1"
	];
	#[cfg(windows)]
	const PORTS: [&'static str; 10] = [
		"COM0", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9"
	];

	#[test]
	fn test() {
		let (conn, port) = PORTS.iter().filter_map(|&port| {
				match Connection::open(&port, 9600).map(|c| (c, port)) {
					Ok(o) => Some(o),
					Err(e) => {
						println!("Error opening connection on port {}: {}", port, e);
						None
					}
				}
			})
			.next()
			.unwrap();
		println!("Serial connection open on port {}", port);
	}
}