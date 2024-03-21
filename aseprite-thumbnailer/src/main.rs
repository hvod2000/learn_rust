use aseprite_reader2::Aseprite;
use image::{self, Rgba};

fn main() {
	let args: Vec<String> = std::env::args().collect();
	save_thumbnail(&args[1], &args[2]);
}

fn save_thumbnail(input: &str, output: &str) {
	let aseprite = Aseprite::from_path(input).expect("Could not read aseprite file.");
	let frames = aseprite.frames();
	let image = &frames.get_for(&(0..1)).get_images().unwrap()[0];
	let w = image.width();
	let h = image.height();
	let s = 128;
	let mut thumbnail = image::ImageBuffer::new(s, s);

	for (x, y, pixel) in thumbnail.enumerate_pixels_mut() {
		let color = image.get_pixel(x * w / s, y * h / s).0;
		*pixel = Rgba(color);
	}
	thumbnail.save(output).expect("Failed to save thumbnail file");
}
