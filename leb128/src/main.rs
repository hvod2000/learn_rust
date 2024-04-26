mod leb128;
use std::io::Result;

use leb128::{Reader, Writer};

fn main() -> Result<()> {
	for x in [123, -123456, 42312344123] {
		let mut buf: Vec<u8> = vec![];
		buf.i128(x)?;
		println!("leb128({}) = {:x?}", x, buf);
		assert!(buf.as_slice().i128()? == x);
	}
	Ok(())
}
