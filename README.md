# se-rs-ial

Cross-platform serial communications library in Rust.

# Usage Example

	use se_rs_ial::{ Connection, BaudRate };

	#[cfg(windows)]
	const PORT: &'static str = "COM1";
	#[cfg(unix)]
	const PORT: &'static str = "/dev/ttyUSB0";

	let mut conn = Connection::open(PORT, BaudRate::B9600).unwrap();

	conn.write("Hello World!\n".as_bytes()).unwrap();

# [Documentation](https://bryal.github.io/se-rs-ial/se_rs_ial/)

# Examples

Check out the [examples](https://github.com/bryal/se-rs-ial/tree/master/examples) directory.

Examples can be run with `cargo run --example <example-name> -- <port>`,
where `port` is the serial port, e.g. `COM3` or `/dev/ttyUSB0`. On some systems, `sudo` be required for sufficient permissions.

E.g. `sudo cargo run --example color_swirl -- /dev/ttyS1`