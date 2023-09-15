mod app_config;
mod wallhaven;

use std::process::exit;

use app_config::*;

fn main() {
    let config = load_config("");
    match config.query {
        Search(..) | Collection { .. } if config.time() > 0 => loop {
            // TODO: loop should pause for seconds and change walls.
            let url = config.geturl();
        },
        Search(..) | Collection { .. } => {
            eprintln!("Time cannot be 0 or less!");
            exit(1);
        }
        ID(..) => {
            // TODO: load wall from API and set it as a wall, if not already.
            let url = config.geturl();
            exit(0);
        }
    }
}
