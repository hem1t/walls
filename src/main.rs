mod app_config;

use app_config::AppConfig;
use std::{fs::File, io::Read, path::PathBuf};

fn main() {}

fn load_config(path: &PathBuf) -> AppConfig {
    let mut file = File::open(path).expect("Fix later file open");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("Fix later file reading to string");
    ron::from_str::<app_config::AppConfig>(&s).expect("Fix later ron_file to AppConfig")
}

#[test]
fn test_printexample() {
    let conf = load_config(&PathBuf::from("example.ron"));
    println!("{:#?}", conf);
}
