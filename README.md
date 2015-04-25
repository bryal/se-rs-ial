# se-rs-ial

Cross-platform serial communications library in Rust.

# Examples

	use se_rs_ial::{ Connection, BaudRate, ByteSize, Parity, StopBits };

	#[cfg(windows)]
	const PORT: &'static str = "COM1";
	#[cfg(unix)]
	const PORT: &'static str = "/dev/ttyUSB0";

	let mut conn = match Connection::open(PORT, BaudRate::B9600).unwrap();

	conn.set_byte_size(ByteSize::B8)
		.and_then(|_| conn.set_parity(Parity::None))
		.and_then(|_| conn.set_stop_bits(StopBits::B1))
		.unwrap()