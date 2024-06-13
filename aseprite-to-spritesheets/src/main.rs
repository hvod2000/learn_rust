#![allow(dead_code, unused_mut, unused_variables, unused_assignments)]
use std::collections::HashMap;
use std::io::{Error, Result};
use std::num::Wrapping;
use std::path::{Path, PathBuf};
use std::process::exit;
mod ase;
use ase::{Aseprite, Tag};
use image::{self, Rgba};

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
		export(&source_dir, &target_dir, &source)?;
	}
	Ok(())
	// save_thumbnail(&args[1], &args[2]);
}

fn export(source_dir: &Path, target_dir: &Path, ase_path: &Path) -> Result<()> {
	println!("{}", ase_path.to_string_lossy());
	let ase = Aseprite::load(ase_path)?;
	let tags = ase.tags.to_vec();
	let frames = ase.frames.to_vec();
	let path = ase_path.strip_prefix(source_dir).unwrap();
	if tags.len() > 0 || frames.len() > 1 {
		let tags = if tags.len() > 0 { tags } else { vec![Tag::idle(frames.len())] };
		let mut frames = shrink_frames(frames);
		let redirection = uniq(&mut frames);
		let (spritesheet, locations) = pack_sprites(&frames);
		let sprite_path = target_dir.join("sprites").join(path).with_extension("png");
		println!(" -> {}", sprite_path.to_string_lossy());
		save_image(&spritesheet, &sprite_path)?;
		let anim_path = target_dir.join("animations").join(path).with_extension("tres");
		println!(" -> {}", anim_path.to_string_lossy());
		let mut anim = String::new();
		anim.push_str(&format!(
			"[gd_resource type=\"SpriteFrames\" load_steps={} format=3]\n\n",
			frames.len() + 2
		));
		anim.push_str(&format!("[ext_resource type=\"Texture2D\" path=\"res://assets/sprites/{}\" id=\"1_wsu57\"]\n\n", &path.with_extension("png").to_string_lossy()));
		for (i, [x, y, w, h]) in locations.into_iter().enumerate() {
			anim.push_str(&format!("[sub_resource type=\"AtlasTexture\" id=\"AtlasTexture_{:05}\"]\natlas = ExtResource(\"1_wsu57\")\n", i + 1));
			anim.push_str(&format!("region = Rect2({}, {}, {}, {})\n\n", x, y, w, h));
		}
		anim.push_str("[resource]\nanimations = [");
		for Tag { name, start, end } in tags {
			anim.push_str("{\n");
			anim.push_str("\"frames\": ["); // TODO: take this info from aseprite file
			for frame in start..end {
				anim.push_str("{\n");
				anim.push_str("\"duration\": 1.0,\n"); // TODO: Actually read duration
				anim.push_str(&format!(
					"\"texture\": SubResource(\"AtlasTexture_{:05}\")\n",
					redirection[frame] + 1
				));
				anim.push_str("}, ");
			}
			anim.push_str("],\n"); // TODO: take this info from aseprite file
			anim.push_str("\"loop\": true,\n"); // TODO: take this info from aseprite file
			anim.push_str(&format!("\"name\": &\"{}\",\n", name.to_lowercase())); // TODO: snake case?
			anim.push_str("\"speed\": 10,\n");
			anim.push_str("}, ");
		}
		anim.push_str("]\n");
		if let Some(p) = anim_path.parent() {
			std::fs::create_dir_all(p)?
		};
		std::fs::write(anim_path, anim)?;
	} else {
		let path = target_dir.join("sprites").join(path).with_extension("png");
		println!(" -> {}", path.to_string_lossy());
		save_image(&frames[0], &path)?;
	}
	Ok(())
}

fn save_image(image: &Vec<Vec<[u8; 4]>>, path: &Path) -> Result<()> {
	let mut img = image::ImageBuffer::new(image[0].len() as u32, image.len() as u32);
	for (x, y, pixel) in img.enumerate_pixels_mut() {
		*pixel = Rgba(image[y as usize][x as usize])
	}
	if let Some(p) = path.parent() {
		std::fs::create_dir_all(p)?
	};
	if img.save(path).is_err() {
		return Err(Error::other("Failed to save spritesheet"));
	}
	Ok(())
}

fn pack_sprites(sprites: &Vec<Vec<Vec<[u8; 4]>>>) -> (Vec<Vec<[u8; 4]>>, Vec<[usize; 4]>) {
	let w = sprites[0][0].len();
	let h = sprites[0].len();
	let mut cols = 1;
	let mut rows = 1;
	while cols * rows < sprites.len() {
		if w * cols + w <= h * rows + h {
			cols += 1;
		} else {
			rows += 1;
		}
	}
	let mut spritesheet = vec![vec![[0; 4]; cols * w]; rows * h];
	let mut locations = vec![[0; 4]; sprites.len()];
	for (i, sprite) in sprites.into_iter().enumerate() {
		let x0 = i % cols * w;
		let y0 = i / cols * h;
		locations[i] = [x0, y0, w, h];
		for y in 0..h {
			for x in 0..w {
				spritesheet[y0 + y][x0 + x] = sprite[y][x];
			}
		}
	}
	(spritesheet, locations)
}

fn shrink_frames(mut frames: Vec<Vec<Vec<[u8; 4]>>>) -> Vec<Vec<Vec<[u8; 4]>>> {
	let w = frames[0][0].len();
	let h = frames[0].len();
	let mut max_dw = 0;
	for dw in 0..((w - 1) / 2) {
		let mut empty_line = true;
		for i in 0..frames.len() {
			for y in 0..h {
				for x in [dw, w - dw - 1] {
					empty_line &= frames[i][y][x][3] == 0;
				}
			}
		}
		if !empty_line {
			break;
		}
		max_dw = dw;
	}
	let mut max_dh = 0;
	for dh in 0..((h - 1) / 2) {
		let mut empty_line = true;
		for i in 0..frames.len() {
			for y in [dh, h - dh - 1] {
				for x in 0..w {
					empty_line &= frames[i][y][x][3] == 0;
				}
			}
		}
		if !empty_line {
			break;
		}
		max_dh = dh;
	}
	let [dw, dh] = [max_dw, max_dh];
	frames = frames
		.into_iter()
		.map(|frame| {
			frame[dh..(h - dh)].into_iter().map(|row| row[dw..(w - dw)].to_vec()).collect()
		})
		.collect();
	frames
}

fn uniq(frames: &mut Vec<Vec<Vec<[u8; 4]>>>) -> Vec<usize> {
	let mut hashes: HashMap<u64, usize> = HashMap::new();
	let mut redirection = vec![0; frames.len()];
	let mut saved_frames = vec![];
	for (i, frame) in frames.into_iter().enumerate() {
		let hash = hash(frame);
		if hashes.contains_key(&hash) {
			redirection[i] = hashes[&hash];
		} else {
			redirection[i] = saved_frames.len();
			hashes.insert(hash, redirection[i]);
			saved_frames.push(frame.to_vec());
		}
	}
	*frames = saved_frames;
	redirection
}

fn hash(image: &Vec<Vec<[u8; 4]>>) -> u64 {
	let mut hash = Wrapping(0u64);
	let factor = Wrapping(0x73A1D61C9611403B);
	for row in image {
		for pixel in row {
			hash = hash * factor + Wrapping(u32::from_le_bytes(*pixel) as u64);
		}
	}
	hash.0
}
