extern crate benten_wayland;
mod logger;

use std::fs::read_to_string;

fn main() {
	logger::init("debug").map_err(|err| eprintln!("logger failed to initialise: {:?}", err)).unwrap();
	let path = xdg::BaseDirectories::with_prefix("benten").unwrap().get_data_home().join("current_mode");

	//read mode at $XDG_DATA_HOME/benten/current_mode, if not set panic
	let mode = read_to_string(&path)
		.map_err(|_| log::error!("No mode set at $XDG_DATA_HOME/benten/current_mode")).unwrap();

	let mut state = benten_wayland::State::new(path, &mode);
	state.run();
}