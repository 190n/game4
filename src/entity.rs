use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub trait Behavior {
	fn tick(&mut self, ent: &mut EntityCommon, dt: f64) -> ();
	fn draw(&self, ent: &EntityCommon, canvas: &mut Canvas<Window>) -> ();
}

pub struct EntityCommon {
	pub x: f64,
	pub y: f64,
}

pub struct Entity(EntityCommon, Box<dyn Behavior>);

impl Entity {
	pub fn tick(&mut self, dt: f64) -> () {
		self.1.tick(&mut self.0, dt);
	}

	pub fn draw(&self, canvas: &mut Canvas<Window>) -> () {
		self.1.draw(&self.0, canvas);
	}
}

pub struct Looper {
	t: f64,
}

impl Looper {
	pub fn new() -> Entity {
		Entity(EntityCommon { x: 0.0, y: 0.0 }, Box::new(Looper { t: 0.0 }))
	}
}

impl Behavior for Looper {
	fn tick(&mut self, ent: &mut EntityCommon, dt: f64) -> () {
		self.t += dt;
		self.t %= 2.0 * std::f64::consts::PI;
		ent.x = f64::cos(self.t) * 100.0 + 400.0;
		ent.y = f64::sin(self.t) * 100.0 + 300.0;
	}

	fn draw(&self, ent: &EntityCommon, canvas: &mut Canvas<Window>) -> () {
		canvas.set_draw_color(Color::RED);
		canvas
			.fill_rect(Rect::new(
				(ent.x - 20.0) as i32,
				(ent.y - 20.0) as i32,
				40,
				40,
			))
			.unwrap();
	}
}
