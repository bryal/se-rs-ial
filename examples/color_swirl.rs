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

//! Color Swirl. Works with an Arduino running LEDstream

#![feature(step_by)]

extern crate se_rs_ial;

use se_rs_ial::*;
use std::{ thread, env };
use std::io::Write;

fn main() {
	let port = env::args().skip(1).next().unwrap();

	let mut connection = Connection::open(&port, BaudRate::B115200).unwrap();

	thread::sleep_ms(1000);

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
		connection.write(&buffer[..])
			.and_then(|_| connection.flush())
			.ok()
			.expect("Write failed");

		// The arduino can't handle it if we go too fast
		thread::sleep_ms(3);
	}
}