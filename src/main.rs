mod app_config;

use app_config::*;

fn main() {
    let config = load_config("");
    match config.query {
        Search(..) | Collection { .. } if config.time() > 0 => loop {
            // TODO: loop should pause for seconds and change walls.
            let url = config.geturl();
        },
        Search(..) | Collection { .. } => panic!("Time cannot be 0 or less!"),
        ID(..) => {
            // TODO: load wall from API and set it as a wall, if not already.
            let url = config.geturl();
        }
    }
}
