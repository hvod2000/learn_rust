use std::io::{Error, Result};
// TODO: Checkout https://crates.io/crates/aseprite-loader

pub struct Aseprite {
	pub width: usize,
	pub height: usize,
	pub tags: Vec<Tag>,
	pub frames: Vec<Vec<Vec<[u8; 4]>>>,

	file: aseprite_reader2::Aseprite,
}

#[derive(Debug, Clone)]
pub struct Tag {
	pub name: String,
	pub start: usize,
	pub end: usize,
}

impl Aseprite {
	pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
		let Ok(file) = aseprite_reader2::Aseprite::from_path(path) else {
			return Err(Error::other("Failed to read aseprite file"));
		};
		let tags = file
			.tags()
			.all()
			.map(|tag| Tag {
				name: tag.name.clone(),
				start: tag.frames.start as usize,
				end: tag.frames.end as usize,
			})
			.collect();
		let frames_count = file.frames().count();
		let frames = file.frames().get_for(&(0..(frames_count as u16))).get_images().unwrap();
		let width = frames[0].width() as usize;
		let height = frames[0].height() as usize;
		let frames =
			Vec::from_iter(frames.into_iter().map(|image| {
				Vec::from_iter((0..height).map(|y| {
					Vec::from_iter((0..width).map(|x| image.get_pixel(x as u32, y as u32).0))
				}))
			}));
		Ok(Aseprite { width, height, tags, frames, file })
	}
}

impl Tag {
	pub fn idle(frames: usize) -> Self {
		Tag { name: "Idle".to_string(), start: 0, end: frames }
	}
}
