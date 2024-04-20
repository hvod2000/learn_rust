use std::f64::consts::TAU;
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::num_traits::Pow;
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const SPEED: f64 = 3.0;
const WORLD_SIZE: f64 = 1000.0;
const SPHERE_RADIUS: f64 = WORLD_SIZE / TAU;
const SCALE: f64 = 3.0;

fn euclidian_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
	let dx = x2 - x1;
	let dy = y2 - y1;
	return (dx * dx + dy * dy).sqrt();
}

fn euclidian_manifold_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
	let w = WORLD_SIZE;
	let h = WORLD_SIZE / 2.0;
	let dx = (x2 - x1 + w / 2.0).rem_euclid(w) - w / 2.0;
	let dy = (y2 - y1 + h / 2.0).rem_euclid(h) - h / 2.0;
	return (dx * dx + dy * dy).sqrt();
}

fn hyperbolical_on_y_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
	let dx = (x2 - x1 + WORLD_SIZE / 2.0).rem_euclid(WORLD_SIZE) - WORLD_SIZE / 2.0;
	let dx = dx * (y2 / WORLD_SIZE * 10.0).cosh();
	return (dx * dx + (y2 - y1).powi(2)).sqrt();
}

// fn hyperbolical_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
// 	let x2 = (x2 - x1) * ((x2.powi(2) + y2.powi(2)).sqrt() / WORLD_SIZE * 10.0).cosh();
// 	let y2 = (y2 - y1) * ((x2.powi(2) + y2.powi(2)).sqrt() / WORLD_SIZE * 10.0).cosh();
// 	return euclidian_distance(0.0, 0.0, x2, y2);
// }

fn hyperbolical_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
	let infinity = WORLD_SIZE as f64 * 2.0 / 20.0;
	let x1 = x1 as f64 / infinity;
	let y1 = y1 as f64 / infinity;
	let z1 = (1.0 + x1.powi(2) + y1.powi(2)).sqrt();
	let x2 = x2 as f64 / infinity;
	let y2 = y2 as f64 / infinity;
	let z2 = (1.0 + x2.powi(2) + y2.powi(2)).sqrt();
	let product: f64 = z1 * z2 - x1 * x2 - y1 * y2;
	// println!("{} -> {}", product, product.max(1.0).acosh());
	return product.max(1.0).acosh() * infinity;
}

fn spherical_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
	const R: f64 = SPHERE_RADIUS;
	let λ1 = x1 / R;
	let φ1 = y1 / R;
	let λ2 = x2 / R;
	let φ2 = y2 / R;
	let x1 = λ1.sin() * φ1.cos();
	let y1 = λ1.cos() * φ1.cos();
	let z1 = φ1.sin();
	let x2 = λ2.sin() * φ2.cos();
	let y2 = λ2.cos() * φ2.cos();
	let z2 = φ2.sin();
	let product = x1 * x2 + y1 * y2 + z1 * z2;
	return product.min(1.0).max(-1.0).acos() * R;
}

fn main() -> tetra::Result {
	ContextBuilder::new("Walker", 1120, 630)
		.resizable(true)
		.quit_on_escape(true)
		.build()?
		.run(GameState::new)
}

struct GameState {
	space_distance: fn(f64, f64, f64, f64) -> f64,
	texture: Texture,
	player: Entity,
	objects: Vec<Entity>,
}

impl State for GameState {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result {
		let d = self.space_distance;
		let mut x = self.player.position.x;
		let mut y = self.player.position.y;
		if input::is_key_down(ctx, Key::Up) || input::is_key_down(ctx, Key::W) {
			y += zero(&|dy| d(x, y, x, y - dy) - SPEED);
		}
		if input::is_key_down(ctx, Key::Left) || input::is_key_down(ctx, Key::A) {
			x -= zero(&|dx| d(x - dx, y, x, y) - SPEED);
		}
		if input::is_key_down(ctx, Key::Down) || input::is_key_down(ctx, Key::S) {
			y -= zero(&|dy| d(x, y, x, y + dy) - SPEED);
		}
		if input::is_key_down(ctx, Key::Right) || input::is_key_down(ctx, Key::D) {
			x += zero(&|dx| d(x + dx, y, x, y) - SPEED);
		}
		if input::is_key_down(ctx, Key::Num1) {
			self.space_distance = euclidian_distance;
		}
		if input::is_key_down(ctx, Key::Num2) {
			self.space_distance = euclidian_manifold_distance;
		}
		if input::is_key_down(ctx, Key::Num3) {
			self.space_distance = spherical_distance;
		}
		if input::is_key_down(ctx, Key::Num4) {
			self.space_distance = hyperbolical_on_y_distance;
		}
		if input::is_key_down(ctx, Key::Num5) {
			self.space_distance = hyperbolical_distance;
		}
		if input::is_key_pressed(ctx, Key::Space) {
			self.objects.push(Entity::new(self.texture.clone(), Vec2::new(x, y)));
		}
		// println!("plyer: {},{}", x, y);
		self.player.position.x = x;
		self.player.position.y = y;
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		let d = self.space_distance;
		graphics::clear(ctx, Color::rgb(0.102, 0.106, 0.149));
		let (w, h) = tetra::window::get_size(ctx);
		let (w, h) = (w as f64, h as f64);
		let x0 = self.player.position.x;
		let y0 = self.player.position.y;
		for obj in &self.objects {
			let x = obj.position.x;
			let y = obj.position.y;

			// let dx = d(x0, y0, x, y0).copysign(d(x0, y0, x, y0) - d(x0 + 0.01, y0, x, y0));
			// let dy = d(x, y0, x, y).copysign(d(x, y0, x, y) - d(x, y0 + 0.01, x, y));

			// let dy = d(x0, y0, x0, y).copysign(d(x0, y0, x0, y) - d(x0, y0 + 0.01, x0, y));
			// let dx = d(x0, y, x, y).copysign(d(x0, y, x, y) - d(x0 + 0.01, y, x, y));

			let (x2, y2) = (x, y);
			let (x1, y1) = ((x2 + x0) / 2.0, (y2 + y0) / 2.0);
			let dy = d(x0, y0, x0, y1).copysign(d(x0, y0, x0, y1) - d(x0, y0 + 0.01, x0, y1));
			let dx = d(x0, y1, x1, y1).copysign(d(x0, y1, x1, y1) - d(x0 + 0.01, y1, x1, y1));
			let dy = dy + d(x1, y1, x1, y2).copysign(d(x1, y1, x1, y2) - d(x1, y1 + 0.01, x1, y2));
			let dx = dx + d(x1, y2, x2, y2).copysign(d(x1, y2, x2, y2) - d(x1 + 0.01, y2, x2, y2));

			// let distance = d(x0, y0, x, y);
			// let ε = 1e-6;
			// let dfdx = -(d(x0 + ε, y0, x, y) - distance) / ε;
			// let dfdy = -(d(x0, y0 + ε, x, y) - distance) / ε;
			// let angle = dfdy.atan2(dfdx);
			// let dx = angle.cos() * distance;
			// let dy = angle.sin() * distance;

			// println!("{} : {} {}", distance, x0, y0);

			let pos = Vec2::new((w / 2.0 + SCALE * dx) as f32, (h / 2.0 - SCALE * dy) as f32);
			obj.texture.draw(ctx, pos);
		}
		self.player.texture.draw(ctx, Vec2::new(w as f32 / 2.0, h as f32 / 2.0));
		Ok(())
	}
}

fn zero(f: &dyn Fn(f64) -> f64) -> f64 {
	unsafe {
		let mut min: u64 = 0;
		// let mut max: u32 = 0x7f800000;
		let mut max: u64 = std::mem::transmute(WORLD_SIZE / 10.0);
		while max > min {
			let mid = (max + min) / 2;
			let x: f64 = std::mem::transmute(mid);
			let y = f(x);
			if y < 0.0 {
				min = mid + 1;
			} else if y == 0.0 {
				return x;
			} else {
				max = mid - 1;
			}
		}
		return std::mem::transmute(min);
	}
}

struct Entity {
	texture: Texture,
	position: Vec2<f64>,
}

impl Entity {
	fn new(texture: Texture, position: Vec2<f64>) -> Entity {
		Entity { texture, position }
	}
}

impl GameState {
	fn new(ctx: &mut Context) -> tetra::Result<GameState> {
		let player_texture = Texture::new(ctx, "./player.png")?;
		let player_position = Vec2::new(0.0, 0.0);
		let mut objects = vec![];
		for x in -10..10 {
			for y in -5..5 {
				let x = 50 * x + 25;
				let y = 50 * y + 25;
				let pos = Vec2::new(x as f64, y as f64);
				let obj = Entity::new(player_texture.clone(), pos);
				objects.push(obj);
			}
		}
		Ok(GameState {
			space_distance: hyperbolical_distance,
			texture: player_texture.clone(),
			player: Entity::new(player_texture.clone(), player_position),
			objects,
		})
	}
}
