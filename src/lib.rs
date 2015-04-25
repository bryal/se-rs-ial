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

use std::io;
#[cfg(windows)]
use std::io::{ Error, ErrorKind };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaudRate {
	B0 = 0,
	B50 = 50,
	B75 = 75,
	B110 = 110,
	B134 = 134,
	B150 = 150,
	B200 = 200,
	B300 = 300,
	B600 = 600,
	B1200 = 1200,
	B1800 = 1800,
	B2400 = 2400,
	B4800 = 4800,
	B9600 = 9600,
	B19200 = 19200,
	B38400 = 38400,
	B57600 = 57600,
	B115200 = 115200,
	B230400 = 230400,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteSize {
	B5,
	B6,
	B7,
	B8,
}

#[cfg(windows)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
	None,
	Even,
	Odd,
	Mark,
	Space,
}

#[cfg(windows)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
	B1,
	B1p5,
	B2,
}

// Windows only
#[cfg(windows)]
pub struct Connection {
	conn: serial_win::Connection
}
#[cfg(windows)]
impl Connection {
	pub fn open(port: &str, baud_rate: BaudRate) -> io::Result<Connection> {
		serial_win::Connection::new(port, baud_rate as u32).map(|conn| Connection { conn: conn })
	}

	pub fn baud_rate(&self) -> io::Result<BaudRate> {
		use BaudRate::*;
		self.conn.baud_rate().and_then(|baud_rate| match baud_rate {
			0 => Ok(B0),
			50 => Ok(B50),
			75 => Ok(B75),
			110 => Ok(B110),
			134 => Ok(B134),
			150 => Ok(B150),
			200 => Ok(B200),
			300 => Ok(B300),
			600 => Ok(B600),
			1200 => Ok(B1200),
			1800 => Ok(B1800),
			2400 => Ok(B2400),
			4800 => Ok(B4800),
			9600 => Ok(B9600),
			19200 => Ok(B19200),
			38400 => Ok(B38400),
			57600 => Ok(B57600),
			115200 => Ok(B115200),
			230400 => Ok(B230400),
			x => Err(Error::new(ErrorKind::Other, format!("Unexpected baud rate returned: {}", x)))
		})
	}

	pub fn set_baud_rate(&mut self, baud_rate: BaudRate) -> io::Result<()> {
		self.conn.set_baud_rate(baud_rate as u32)
	}

	pub fn byte_size(&self) -> io::Result<ByteSize> {
		self.conn.byte_size().and_then(|byte_size| match byte_size {
			5 => Ok(ByteSize::B5),
			6 => Ok(ByteSize::B6),
			7 => Ok(ByteSize::B7),
			8 => Ok(ByteSize::B8),
			x => Err(Error::new(ErrorKind::Other, format!("Unexpected byte size returned: {}", x)))
		})
	}

	pub fn set_byte_size(&mut self, byte_size: ByteSize) -> io::Result<()> {
		self.conn.set_byte_size(match byte_size {
			ByteSize::B5 => 5,
			ByteSize::B6 => 6,
			ByteSize::B7 => 7,
			ByteSize::B8 => 8,
		})
	}

	pub fn parity(&self) -> io::Result<Parity> {
		use serial_win::Parity::*;
		self.conn.parity().map(|parity| match parity {
			NO => Parity::None,
			EVEN => Parity::Even,
			ODD => Parity::Odd,
			MARK => Parity::Mark,
			SPACE => Parity::Space,
		})
	}

	pub fn set_parity(&mut self, parity: Parity) -> io::Result<()> {
		use serial_win::Parity::*;
		self.conn.set_parity(match parity {
			Parity::None => NO,
			Parity::Even => EVEN,
			Parity::Odd => ODD,
			Parity::Mark => MARK,
			Parity::Space => SPACE,
		})
	}

	pub fn stop_bits(&self) -> io::Result<StopBits> {
		use serial_win::StopBits::*;
		self.conn.stop_bits().map(|stop_bits| match stop_bits {
			ONE => StopBits::B1,
			ONE5 => StopBits::B1p5,
			TWO => StopBits::B2,
		})
	}

	pub fn set_stop_bits(&mut self, stop_bits: StopBits) -> io::Result<()> {
		use serial_win::StopBits::*;
		self.conn.set_stop_bits(match stop_bits {
			StopBits::B1 => ONE,
			StopBits::B1p5 => ONE5,
			StopBits::B2 => TWO,
		})
	}
}

#[cfg(unix)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
	None,
	Even,
	Odd,
}

#[cfg(unix)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
	B1,
	B2,
}

// Unix-like only
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
	pub fn open(port: &str, baud_rate: BaudRate) -> io::Result<Connection> {
		serial::OpenOptions::new()
			.read(true).write(true)
			.open(port)
			.map(|serial_port| Connection{ conn: serial_port })
			.and_then(|mut conn| conn.set_baud_rate(baud_rate)
				.and_then(|_| conn.set_byte_size(ByteSize::B8))
				.and_then(|_| conn.set_parity(Parity::None))
				.and_then(|_| conn.set_stop_bits(StopBits::B1).map(|_| conn)))
	}

	pub fn baud_rate(&self) -> io::Result<BaudRate> {
		use serial::BaudRate as SBaudRate;
		use BaudRate::*;
		self.conn.baud_rate().map(|(_, out_baud)| match out_baud {
			SBaudRate::B0 => B0,
			SBaudRate::B50 => B50,
			SBaudRate::B75 => B75,
			SBaudRate::B110 => B110,
			SBaudRate::B134 => B134,
			SBaudRate::B150 => B150,
			SBaudRate::B200 => B200,
			SBaudRate::B300 => B300,
			SBaudRate::B600 => B600,
			SBaudRate::B1200 => B1200,
			SBaudRate::B1800 => B1800,
			SBaudRate::B2400 => B2400,
			SBaudRate::B4800 => B4800,
			SBaudRate::B9600 => B9600,
			SBaudRate::B19200 => B19200,
			SBaudRate::B38400 => B38400,
			SBaudRate::B57600 => B57600,
			SBaudRate::B115200 => B115200,
			SBaudRate::B230400 => B230400,
		})
	}

	pub fn set_baud_rate(&mut self, baud_rate: BaudRate) -> io::Result<()> {
		use serial::BaudRate as SBaudRate;
		use BaudRate::*;
		self.conn.set_baud_rate(serial::Direction::Both, match baud_rate {
			B0 => SBaudRate::B0,
			B50 => SBaudRate::B50,
			B75 => SBaudRate::B75,
			B110 => SBaudRate::B110,
			B134 => SBaudRate::B134,
			B150 => SBaudRate::B150,
			B200 => SBaudRate::B200,
			B300 => SBaudRate::B300,
			B600 => SBaudRate::B600,
			B1200 => SBaudRate::B1200,
			B1800 => SBaudRate::B1800,
			B2400 => SBaudRate::B2400,
			B4800 => SBaudRate::B4800,
			B9600 => SBaudRate::B9600,
			B19200 => SBaudRate::B19200,
			B38400 => SBaudRate::B38400,
			B57600 => SBaudRate::B57600,
			B115200 => SBaudRate::B115200,
			B230400 => SBaudRate::B230400,
		})
	}

	pub fn byte_size(&self) -> io::Result<ByteSize> {
		use serial::DataBits::*;
		self.conn.data_bits().map(|byte_size| match byte_size {
			Five => ByteSize::B5,
			Six => ByteSize::B6,
			Seven => ByteSize::B7,
			Eight => ByteSize::B8,
		})
	}

	pub fn set_byte_size(&mut self, byte_size: ByteSize) -> io::Result<()> {
		use serial::DataBits::*;
		self.conn.set_data_bits(match byte_size {
			ByteSize::B5 => Five,
			ByteSize::B6 => Six,
			ByteSize::B7 => Seven,
			ByteSize::B8 => Eight,
		})
	}

	pub fn parity(&self) -> io::Result<Parity> {
		use serial::Parity as SParity;
		self.conn.parity().and_then(|parity| match parity {
			SParity::None => Ok(Parity::None),
			SParity::Even => Ok(Parity::Even),
			SParity::Odd => Ok(Parity::Odd),
		})
	}

	pub fn set_parity(&mut self, parity: Parity) -> io::Result<()> {
		use serial::Parity as SParity;
		self.conn.set_parity(match parity {
			Parity::None => SParity::None,
			Parity::Even => SParity::Even,
			Parity::Odd => SParity::Odd,
		})
	}

	pub fn stop_bits(&self) -> io::Result<StopBits> {
		use serial::StopBits::*;
		self.conn.stop_bits().map(|stop_bits| match stop_bits {
			One => StopBits::B1,
			Two => StopBits::B2,
		})
	}

	pub fn set_stop_bits(&mut self, stop_bits: StopBits) -> io::Result<()> {
		use serial::StopBits::*;
		self.conn.set_stop_bits(match stop_bits {
			StopBits::B1 => One,
			StopBits::B2 => Two,
		})
	}
}

impl io::Write for Connection {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.conn.write(buf)
	}

	fn flush(&mut self) -> io::Result<()> {
		self.conn.flush()
	}
}

impl io::Read for Connection {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.conn.read(buf)
	}
}

// Common implementations
impl Connection {

}

#[cfg(test)]
mod tests {
	use super::*;

	// Change these to whatever works on system of test
	#[cfg(unix)]
	const PORT: &'static str = "/dev/ttyUSB0";
	#[cfg(windows)]
	const PORT: &'static str = "COM8";

	// Multiple threads can't access same serial port at the same time. Run sequentially
	#[test]
	fn sequential_tests() {
		test_find_and_open();
		test_configure();
	}

	fn test_find_and_open() {
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

		let (_, port) = PORTS.iter().filter_map(|&port| {
				match Connection::open(&port, BaudRate::B9600).map(|c| (c, port)) {
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

	fn test_configure() {
		let mut conn = Connection::open(PORT, BaudRate::B9600).unwrap();

		assert_eq!(conn.baud_rate().unwrap(), BaudRate::B9600);
		
		conn.set_baud_rate(BaudRate::B115200)
			.and_then(|_| conn.set_byte_size(ByteSize::B7))
			.and_then(|_| conn.set_parity(Parity::Odd))
			.and_then(|_| conn.set_stop_bits(StopBits::B2))
			.unwrap();

		assert_eq!(conn.baud_rate().unwrap(), BaudRate::B115200);
		assert_eq!(conn.byte_size().unwrap(), ByteSize::B7);
		assert_eq!(conn.parity().unwrap(), Parity::Odd);
		assert_eq!(conn.stop_bits().unwrap(), StopBits::B2);
	}
}