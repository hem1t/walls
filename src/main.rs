mod app_config;
mod wallhaven;

use std::{process::exit, thread, time::Duration};

use app_config::*;
use wallhaven::{get_image, make_request};

fn main() {
    let mut config = load_config("config.ron");
    println!("Loaded Config Successfully!");
    match config.query {
        Search(..) | Collection { .. } if config.time() > 0 => loop {
            // TODO: loop should pause for seconds and change walls.
            println!("In Search and Collection loop");
            let url = config.geturl();
            println!("Config Url: {}", &url);
            if let Ok(data) = make_request(&url) {
                println!("Found Ok(data) of length: {}", &data.data.len());
                println!("Meta: {:#?}", &data.meta);
                if data.data.is_empty() {
                    eprintln!("Not Found any image for the config!");
                    exit(1);
                }
                for img in data.data {
                    println!("Loading: {}", &img.path);
                    config.run_script(&get_image(&img.path).unwrap());
                    println!("Done Setting wall now waiting");
                    thread::sleep(Duration::from_secs(config.time().into()));
                }
                config.filters.inc_page();
            }
        },
        Search(..) | Collection { .. } => {
            eprintln!("Time cannot be 0 or less!");
            exit(1);
        }
        ID(..) => {
            // TODO: load wall from API and set it as a wall, if not already.
            let url = config.geturl();
            if let Ok(data) = make_request(&url) {
                if data.data.is_empty() {
                    eprintln!("Data for ID not Found");
                    exit(1);
                }
                config.run_script(&data.data[0].path);
            }
            exit(0);
        }
    }
}
