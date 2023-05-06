use notan::draw::Draw;

mod backdrop;
mod fps;
mod ship;

pub use backdrop::Backdrop;
pub use fps::Fps;
pub use ship::Ship;

pub trait Behavior {
	fn tick(&mut self, ent: &mut EntityCommon, dt: f32) -> ();
	fn draw(&self, ent: &EntityCommon, draw: &mut Draw) -> ();
}

pub struct EntityCommon {
	pub x: f32,
	pub y: f32,
}

pub struct Entity(pub EntityCommon, pub Box<dyn Behavior>);

impl Entity {
	pub fn tick(&mut self, dt: f32) -> () {
		self.1.tick(&mut self.0, dt);
	}

	pub fn draw(&self, draw: &mut Draw) -> () {
		self.1.draw(&self.0, draw);
	}
}
