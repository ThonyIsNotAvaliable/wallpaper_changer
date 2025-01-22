use std::env;
use wallpaper;

fn main() {
    let args: Vec<String> = env::args().collect();
    wallpaper::set_from_url(&args[1]).unwrap();
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    println!("{:?}", wallpaper::get());
}
