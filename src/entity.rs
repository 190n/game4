use notan::draw::*;

pub trait Behavior {
	fn tick(&mut self, ent: &mut EntityCommon, dt: f32) -> ();
	fn draw(&self, ent: &EntityCommon, canvas: &mut Draw) -> ();
}

pub struct EntityCommon {
	pub x: f64,
	pub y: f64,
}

pub struct Entity(EntityCommon, Box<dyn Behavior>);

impl Entity {
	pub fn tick(&mut self, dt: f32) -> () {
		self.1.tick(&mut self.0, dt);
	}

	pub fn draw(&self, canvas: &mut Draw) -> () {
		self.1.draw(&self.0, canvas);
	}
}

pub struct Looper {
	t: f32,
}

impl Looper {
	pub fn new() -> Entity {
		Entity(EntityCommon { x: 0.0, y: 0.0 }, Box::new(Looper { t: 0.0 }))
	}
}

impl Behavior for Looper {
	fn tick(&mut self, ent: &mut EntityCommon, dt: f32) -> () {
		self.t += dt;
		self.t %= 2.0 * std::f32::consts::PI;
		ent.x = f64::cos(self.t as f64) * 100.0 + 400.0;
		ent.y = f64::sin(self.t as f64) * 100.0 + 300.0;
	}

	fn draw(&self, ent: &EntityCommon, canvas: &mut Draw) -> () {
		canvas.rect((ent.x as f32 - 20.0, ent.y as f32 - 20.0), (40.0, 40.0));
	}
}
