extern crate benten_wayland;
mod logger;

use inotify::{ Inotify, WatchMask };
use futures::{ Stream, StreamExt };

use std::thread;

fn main() {
    logger::init("debug").map_err(|err| eprintln!("logger failed to initialise: {:?}", err)).unwrap();
    let mut server = benten_wayland::Server::new("japanese");
    server.start();

	// std::thread::spawn(|| {
	// 	let mut watcher = Inotify::init().unwrap();
	//     let path = xdg::BaseDirectories::with_prefix("benten").unwrap().get_data_home().join("current_mode");
	// 	watcher.add_watch(&path, WatchMask::MODIFY).unwrap();
	// 	let mut buffer = [0; 1024];

	// 	let events = watcher.read_events_blocking(&mut buffer).unwrap();
	// 	for event in events {
	// 	    println!("event: {:?}", event);
	// 	    server.context.engine.set_mode("japanese");
	// 	}
 //    });
}