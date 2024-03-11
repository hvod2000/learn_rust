extern crate sdl2;
use sdl2::keyboard::{Keycode, Scancode};

pub fn main() {
	sdl2::init().unwrap().video().unwrap();
	let keyboard = [
		vec![
			Scancode::Q,
			Scancode::W,
			Scancode::E,
			Scancode::R,
			Scancode::T,
			Scancode::Y,
			Scancode::U,
			Scancode::I,
			Scancode::O,
			Scancode::P,
			Scancode::LeftBracket,
			Scancode::K,
		],
		vec![
			Scancode::A,
			Scancode::S,
			Scancode::D,
			Scancode::F,
			Scancode::G,
			Scancode::H,
			Scancode::J,
			Scancode::RightBracket,
			Scancode::L,
			Scancode::Semicolon,
			Scancode::Apostrophe,
		],
		vec![
			Scancode::Z,
			Scancode::X,
			Scancode::C,
			Scancode::V,
			Scancode::B,
			Scancode::N,
			Scancode::M,
			Scancode::from_i32('<' as i32).unwrap(),
			Scancode::from_i32('>' as i32).unwrap(),
			Scancode::from_i32('?' as i32).unwrap(),
		],
	];
	for row in keyboard {
		let none = Keycode::from_i32('?' as i32).unwrap();
		let row: Vec<String> =
			row.into_iter().map(|n| Keycode::from_scancode(n).unwrap_or(none).name()).collect();
		println!("[ {} ]", row.join(" ]  [ "));
	}
}
