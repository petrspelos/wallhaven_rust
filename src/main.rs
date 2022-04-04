use clap::Parser;
use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Deserialize)]
struct Wallpapers {
    data: Vec<Wallpaper>,
}

#[derive(Deserialize)]
struct Wallpaper {
    path: String,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    download_only: bool,
}

fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    let mut path = String::new();
    get_random_wallpaper_path(&mut path);

    println!("{}", path);

    if args.download_only {
        return Ok(());
    }

    set_wallpaper(path);

    Ok(())
}

fn get_random_wallpaper_path(path: &mut String) {
    let new_wallpapers = get_new_wallpapers().unwrap();
    let picked_wallpaper = new_wallpapers.data.choose(&mut rand::thread_rng()).unwrap();
    path.push_str(&download_image(&picked_wallpaper.path).unwrap());
}

fn get_new_wallpapers() -> Result<Wallpapers, reqwest::Error> {
    let api_url = "https://wallhaven.cc/api/v1/search";
    Ok(reqwest::blocking::get(api_url)?.json()?)
}

fn download_image(path: &String) -> Result<String, reqwest::Error> {
    let extension = &path[path.len() - 3..];

    let image_format: image::ImageFormat = match extension {
        "png" => image::ImageFormat::Png,
        "jpg" => image::ImageFormat::Jpeg,
        _ => image::ImageFormat::Jpeg,
    };

    let img_bytes = reqwest::blocking::get(path)?.bytes()?;
    let wallpaper_path = format!("wallpaper.{}", extension);
    image::load_from_memory(&img_bytes)
        .unwrap()
        .save_with_format(&wallpaper_path, image_format)
        .unwrap();

    Ok(wallpaper_path)
}

fn set_wallpaper(path: String) {
    let path = format!("{}\\{}", std::env::current_dir().unwrap().display(), path);

    wallpaper::set_from_path(&path).unwrap();
}
