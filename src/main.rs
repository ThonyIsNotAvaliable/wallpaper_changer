use wallpaper;

fn main() {
    wallpaper::set_from_url("https://raw.githubusercontent.com/ThonyIsNotAvaliable/troll-wallpapers/main/wallpapers/dog.jpg").unwrap();
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    println!("{:?}", wallpaper::get());
}
