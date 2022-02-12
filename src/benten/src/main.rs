extern crate benten_wayland;
mod logger;

use std::fs::read_to_string;

fn main() {
	logger::init("debug").map_err(|err| eprintln!("logger failed to initialise: {:?}", err)).unwrap();
	let path = xdg::BaseDirectories::with_prefix("benten").unwrap().get_data_home().join("current_layout");

	//read layout at $XDG_DATA_HOME/benten/current_layout, if not set panic
	let layout = read_to_string(&path)
		.map_err(|_| log::error!("No layout set at $XDG_DATA_HOME/benten/current_layout")).unwrap();

	let mut state = benten_wayland::State::new(path, &layout);
	state.run();
}