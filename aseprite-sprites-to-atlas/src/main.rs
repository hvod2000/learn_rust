#![allow(dead_code, unused_variables, unused_imports, unused_mut)]
use aseprite_reader2::Aseprite;
use image::{self, ImageBuffer, Rgba};
use std::{fs, path::Path, process};

struct Sprite {
	name: String,
	width: usize,
	height: usize,
	frames: Vec<Vec<[u8; 4]>>,
	animations: Vec<(String, Vec<(usize, usize)>)>,
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() != 3 {
		eprint!("Usage: {} SPRITEDIR ATLAS", args[0]);
		process::exit(1);
	}
	let mut sprites = load_sprites(&args[1..2]);
	for sprite in &mut sprites {
		sprite.name = sprite.name.strip_prefix(&args[1]).unwrap().to_string();
		sprite.name = sprite.name.strip_suffix(".ase").unwrap_or(&sprite.name).to_string();
		sprite.name = sprite.name.strip_suffix(".aseprite").unwrap_or(&sprite.name).to_string();
		// sprite.name = sprite.name.to_lowercase().replace(" ", "_");
	}
	let (w, h, atlas_pixels) = pack_sprites(sprites);
	let mut atlas = ImageBuffer::new(w as u32, h as u32);
	for (x, y, pixel) in atlas.enumerate_pixels_mut() {
		let (x, y) = (x as usize, y as usize);
		*pixel = Rgba(atlas_pixels[x + y * w]);
	}
	atlas.save(&args[2]).unwrap();
}

fn pack_sprites(mut sprites: Vec<Sprite>) -> (usize, usize, Vec<[u8; 4]>) {
	// TODO: implement something more clever using answers from here:
	// https://gamedev.stackexchange.com/questions/2829/texture-packing-algorithm
	sprites.sort_by_key(|sprite| sprite.name.clone());
	let cell_w = sprites.iter().map(|s| s.width).max().unwrap_or(1);
	let cell_h = sprites.iter().map(|s| s.height).max().unwrap_or(1);
	let frames = sprites
		.iter()
		.map(|s| s.animations.iter().map(|(_, anim)| anim.len()).sum::<usize>())
		.sum();
	let columns = (1usize..).take_while(|x| x * x < frames).last().unwrap_or(0) + 1;
	let rows = (frames + columns - 1) / columns;
	let atlas_w = cell_w * columns;
	let atlas_h = cell_h * rows;
	let mut atlas = vec![[0, 0, 0, 255]; atlas_w * atlas_h];
	let mut free_space = 0usize;
	for sprite in sprites {
		println!("{:?}: {{", sprite.name);
		println!("\tw: {},", sprite.width);
		println!("\th: {},", sprite.height);
		for (name, frames) in sprite.animations {
			println!("\t{} = {{", name);
			for (frame_id, delay) in frames {
				let delay_in_jiffy = (delay as f64 / 1000. * 240.).round() as i32;
				if delay_in_jiffy == 0 {
					continue;
				}
				let x0 = free_space % columns * cell_w;
				let y0 = free_space / columns * cell_h;
				let pivot_x = cell_w / 2;
				let pivot_y = cell_h / 2;
				let w = sprite.width;
				let h = sprite.height;
				let frame = &sprite.frames[frame_id];
				for y in 0..h {
					for x in 0..w {
						atlas[x0 + x + (y0 + y) * atlas_w] = frame[x + y * sprite.width];
					}
				}
				println!(
					"\t\t{{ t: {}, w: {}, h: {}, px: {}, py: {}, x: {}, y: {} }},",
					delay_in_jiffy, w, h, pivot_x, pivot_y, x0, y0
				);
				// eprintln!("\t\t{}", "─".repeat(80 - 4 * 2));
				free_space += 1;
			}
			println!("\t}},");
		}
		println!("}},");
	}
	(atlas_w, atlas_h, atlas)
}

fn load_sprites(paths: &[String]) -> Vec<Sprite> {
	let mut sprites: Vec<Sprite> = vec![];
	for path in paths {
		if Path::new(path).is_dir() {
			for path in fs::read_dir(path).unwrap() {
				let path = path.unwrap().path().to_string_lossy().to_string();
				if !path.rsplit("/").next().unwrap().starts_with(".") {
					sprites.extend(load_sprites(&[path]));
				}
			}
			continue;
		}
		let aseprite = Aseprite::from_path(path).expect("could not read aseprite file.");
		let image = &aseprite.frames().get_for(&(0..1)).get_images().unwrap()[0];
		let w = image.width() as usize;
		let h = image.height() as usize;
		let mut frames = vec![];
		for i in 0..aseprite.frames().count() {
			let i = i as u16;
			let frame = &aseprite.frames().get_for(&(i..(i + 1))).get_images().unwrap()[0];
			let mut pixels = vec![[0, 0, 0, 0]; w * h];
			for (x, y, pixel) in frame.enumerate_pixels() {
				let (x, y) = (x as usize, y as usize);
				pixels[x + y * w] = pixel.0;
			}
			frames.push(pixels);
		}
		let mut animations = vec![];
		for tag in aseprite.tags().all().collect::<Vec<_>>() {
			let delays = aseprite
				.frames()
				.get_for(&tag.frames)
				.get_infos()
				.unwrap()
				.into_iter()
				.map(|f| f.delay_ms)
				.collect::<Vec<_>>();
			let images = tag.frames.clone().map(|f| f as usize);
			animations.push((tag.name.clone(), images.into_iter().zip(delays).collect()));
		}
		animations.sort_by_key(|anim| anim.0.to_string());
		// TODO: add Idle animation if it is not present
		let sprite = Sprite { name: path.to_string(), width: w, height: h, frames, animations };
		sprites.push(sprite);
	}
	sprites
}
