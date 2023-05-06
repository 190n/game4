use notan::{draw::*, prelude::*};

use manager::Manager;

mod entity;
mod manager;

#[notan_main]
fn main() -> Result<(), String> {
	notan::init_with(Manager::new)
		.add_config(DrawConfig)
		.add_config(
			notan::app::WindowConfig::new()
				.vsync(true)
				.title("game4")
				.high_dpi(true),
		)
		.draw(Manager::draw)
		.initialize(initialize)
		.build()
}

fn initialize(app: &mut App) {
	app.window().set_size(640, 640);
}
