use std::{fs::File, io::prelude::*, path::PathBuf};

use reqwest::blocking::get;
use reqwest::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub id: String,
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    current_page: u32,
    last_page: u32,
    total: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub data: Vec<Image>,
    pub meta: Meta,
}

pub fn make_request(url: String) -> Result<Data, Error> {
    Ok(get(url)?.json::<Data>()?)
}

pub fn get_image(url: String) -> Result<String, Error> {
    let path = format!("./walls/{}", url.get((url.len() - 10)..).unwrap());
    if let Ok(mut file) = File::create(path.clone()) {
        let image = get(url)?.bytes()?;
        file.write_all(&image).unwrap();
    }
    Ok(path)
}
