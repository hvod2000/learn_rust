#![allow(dead_code, unused_mut, unused_variables, unused_assignments)]
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::exit;
mod ase;

use ase::Aseprite;
use image::{self, Rgba};

use crate::ase::Tag;

fn main() -> Result<()> {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 || args.len() > 3 {
		eprintln!("Usage: {} SOURCE_DIR [TARGET_DIR]", args[0]);
		exit(1);
	}
	let source_dir = Path::new(&args[1]).canonicalize().unwrap();
	let target_dir = if args.len() == 3 {
		PathBuf::from(&args[2]).canonicalize().unwrap()
	} else if source_dir.parent().is_some_and(|p| p.ends_with("assets")) {
		source_dir.parent().unwrap().to_path_buf()
	} else {
		source_dir.clone()
	};
	for source in source_dir.read_dir().unwrap() {
		let Ok(source) = source else { continue };
		let source = source.path();
		if !source.extension().is_some_and(|ext| ext == "ase" || ext == "aseprite") {
			continue;
		}
		let path = source.strip_prefix(&source_dir).unwrap();
		let target = target_dir.join(&path);
		// println!("{} -> {}", source.to_string_lossy(), target.to_string_lossy());
		export(&source_dir, &source)?;
	}
	Ok(())
	// save_thumbnail(&args[1], &args[2]);
}

fn export(source_dir: &Path, ase_path: &Path) -> Result<()> {
	println!("{}", ase_path.to_string_lossy());
	let ase = Aseprite::load(ase_path)?;
	let tags = ase.tags.to_vec();
	let frames = ase.frames.to_vec();
	if tags.len() > 0 || frames.len() > 0 {
		let tags = if tags.len() > 0 { tags } else { vec![Tag::idle(frames.len())] };
		println!("\tAnimation with {} tags and {} frames", tags.len(), frames.len());
	} else {
		println!("static sprite");
	}
	Ok(())
}

// fn save_thumbnail(input: &str, output: &str) {
// 	let aseprite = Aseprite::from_path(input).expect("Could not read aseprite file.");
// 	let frames = aseprite.frames();
// 	let image = &frames.get_for(&(0..1)).get_images().unwrap()[0];
// 	let w = image.width();
// 	let h = image.height();
// 	let s = 128;
// 	let mut thumbnail = image::ImageBuffer::new(s, s);
//
// 	for (x, y, pixel) in thumbnail.enumerate_pixels_mut() {
// 		let color = image.get_pixel(x * w / s, y * h / s).0;
// 		*pixel = Rgba(color);
// 	}
// 	thumbnail.save(output).expect("Failed to save thumbnail file");
// }
