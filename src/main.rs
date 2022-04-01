use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Deserialize)]
struct Wallpapers {
    data: Vec<Wallpaper>,
}

#[derive(Deserialize)]
struct Wallpaper {
    purity: String,
    id: String,
    path: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let walls = get_wallpapers().await.expect("");

    let picked_wallpaper = match walls.data.choose(&mut rand::thread_rng()) {
        Some(wall) => wall,
        None => panic!("whoa"),
    };

    let extension = &picked_wallpaper.path[picked_wallpaper.path.len() - 3..];
    let image_format: image::ImageFormat = match &extension {
        &"png" => image::ImageFormat::Png,
        &"jpg" => image::ImageFormat::Jpeg,
        _ => image::ImageFormat::Jpeg,
    };
    let img_bytes = reqwest::blocking::get(&picked_wallpaper.path)?.bytes()?;
    let wallpaper_path = format!("wallpaper.{}", extension);
    image::load_from_memory(&img_bytes)
        .expect("to work")
        .save_with_format(&wallpaper_path, image_format)
        .expect("to work");

    let full_wallpaper_path = format!(
        "{}\\{}",
        std::env::current_dir().unwrap().display(),
        wallpaper_path
    );
    println!("about to set {}", full_wallpaper_path);

    wallpaper::set_from_path(&full_wallpaper_path).unwrap();

    println!(
        "Set the following wallpaper: {} - {} - {}",
        picked_wallpaper.id, picked_wallpaper.purity, picked_wallpaper.path
    );

    Ok(())
}

async fn get_wallpapers() -> Result<Wallpapers, reqwest::Error> {
    let api_url = "https://wallhaven.cc/api/v1/search";

    let wallpapers: Wallpapers = reqwest::blocking::get(api_url)?.json()?;

    Ok(wallpapers)
}
