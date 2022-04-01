use clap::Parser;
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

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    download_only: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    let api_url = "https://wallhaven.cc/api/v1/search";
    let walls: Wallpapers = reqwest::blocking::get(api_url)?.json()?;

    let picked_wallpaper = walls.data.choose(&mut rand::thread_rng()).unwrap();

    let extension = &picked_wallpaper.path[picked_wallpaper.path.len() - 3..];

    let image_format: image::ImageFormat = match &extension {
        &"png" => image::ImageFormat::Png,
        &"jpg" => image::ImageFormat::Jpeg,
        _ => image::ImageFormat::Jpeg,
    };

    let img_bytes = reqwest::blocking::get(&picked_wallpaper.path)?.bytes()?;
    let wallpaper_path = format!("wallpaper.{}", extension);
    image::load_from_memory(&img_bytes)
        .unwrap()
        .save_with_format(&wallpaper_path, image_format)
        .unwrap();

    if args.download_only {
        return Ok(());
    }

    let full_wallpaper_path = format!(
        "{}\\{}",
        std::env::current_dir().unwrap().display(),
        wallpaper_path
    );

    wallpaper::set_from_path(&full_wallpaper_path).unwrap();

    println!(
        "Set the following wallpaper: {} - {} - {}",
        picked_wallpaper.id, picked_wallpaper.purity, picked_wallpaper.path
    );

    Ok(())
}
