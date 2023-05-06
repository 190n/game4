use notan::{draw::*, prelude::*};

use config::WINDOW_SIZE;
use manager::Manager;

mod config;
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
	app.window().set_size(WINDOW_SIZE, WINDOW_SIZE);
}
