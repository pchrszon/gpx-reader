use std::env;
use std::process;

use gpx_reader::Track;

fn main() {
    let mut args = env::args();

    let prog_name = args.next().unwrap();
    let path = match args.next() {
        Some(path) => path,
        None => {
            println!("Usage: {} <GPX-FILE>", prog_name);
            process::exit(1)
        }
    };

    match Track::from_gpx_file(&path) {
        Ok(track) => println!("track length: {:.*}km", 2, track.length()),
        Err(e) => println!("{}", e),
    }
}
