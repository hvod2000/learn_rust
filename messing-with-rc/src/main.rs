use std::{fmt::Display, rc::Rc};

struct Bulbalka {
	name: Box<str>,
}

impl Bulbalka {
	fn new(name: impl AsRef<str>) -> Self {
		let this = Self { name: name.as_ref().into() };
		println!("{} was born", this);
		this
	}
}

impl Display for Bulbalka {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let id = (self.name.as_ptr() as u64).wrapping_mul(0x36A23B45) as u32;
		f.write_str(&self.name)?;
		f.write_str(&format!("[{:X}]", id))
	}
}

impl Clone for Bulbalka {
	fn clone(&self) -> Self {
		let new_one = Self { name: self.name.clone() };
		println!("{} gave birth to {}", self, new_one);
		new_one
	}
}

fn main() {
	let b1 = Rc::new(Bulbalka::new("B"));
	let mut b2 = b1.clone();
	let b3 = b2.clone();
	println!("b1 = {}", b1);
	println!("b2 = {}", b2);
	println!("b3 = {}", b3);
	Rc::make_mut(&mut b2).name = "Σ".into();
	println!("b1 = {}", b1);
	println!("b2 = {}", b2);
	println!("b3 = {}", b3);
}
