use reqwest::blocking::get;
use reqwest::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    id: String,
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    current_page: u32,
    last_page: u32,
    total: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    data: Vec<Image>,
    meta: Meta,
}

pub fn make_request(url: String) -> Result<Data, Error> {
    Ok(get(url)?.json::<Data>()?)
}
