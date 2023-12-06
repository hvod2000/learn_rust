use clap::{Parser, Subcommand};
use std::fmt;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Convert font from one format to another
	Convert { source: PathBuf, target: PathBuf },
	/// Show statistics about main font
	Benchmark {},
}

#[derive(Copy, Clone, PartialEq)]
struct Char {
	bits: u128,
}

#[derive(PartialEq)]
struct Font {
	chars: [Char; 94],
}

impl Font {
	fn new() -> Font {
		Font { chars: [Char { bits: 0 }; 94] }
	}

	fn from_pixels(pixels: &[u8], img_w: u32, img_h: u32) -> Font {
		let (w, h) = (img_w / 47, img_h / 2);
		let mut font = Font::new();
		for i in 0..94 {
			let x0 = i % 47 * w;
			let y0 = i / 47 * h;
			let mut bits = 0;
			for y in y0..(y0 + h) {
				for x in x0..(x0 + w) {
					let j = (4 * (x + y * img_w)) as usize;
					let bit = (y - y0) * w + (x - x0);
					if pixels[j..j + 4] != pixels[0..4] {
						bits |= 1 << bit;
					}
				}
			}
			font.chars[i as usize].bits = bits;
		}
		font
	}

	fn from_png<R: Read>(reader: R) -> Font {
		let png = png::Decoder::new(reader);
		let mut png = png.read_info().unwrap();
		let mut buf = vec![0; png.output_buffer_size()];
		let info = png.next_frame(&mut buf).unwrap();
		let bytes = &buf[..info.buffer_size()];
		let w = info.width;
		let h = info.height;
		Font::from_pixels(bytes, w, h)
	}
}

impl fmt::Display for Char {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for y in 0..16 {
			for x in 0..8 {
				if (self.bits >> (x + y * 8)) & 1 == 1 {
					write!(f, "██")?;
				} else {
					write!(f, "  ")?;
				}
			}
			write!(f, "\n")?;
		}
		return Ok(());
	}
}

fn main() {
	let cli = Cli::parse();
	match &cli.command {
		Commands::Convert { source, target } => {
			todo!("Converting from {source:?} to {target:?}");
		}
		Commands::Benchmark {} => {
			let font = Font::from_png(include_bytes!("font.png") as &[u8]);
			for i in 0..94 {
				let char = char::from_u32(i + 33).unwrap();
				let char_art = font.chars[i as usize];
				println!("char[{i}] = {char}\n{char_art}");
			}
			todo!("Benchmark");
		}
	}
}
