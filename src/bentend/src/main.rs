extern crate benten_wayland;
mod logger;

fn main() {
    logger::init("debug").map_err(|err| eprintln!("logger failed to initialise: {:?}", err)).unwrap();
    benten_wayland::init();
}