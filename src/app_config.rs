use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Category {
    General,
    Anime,
    People,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Purity {
    SFW,
    Sketchy,
    NSFW,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Sorting {
    DateAdded,
    Relevance,
    Random,
    Views,
    Favorites,
    Toplist,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Order {
    Desc,
    Asc,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TopRange {
    OneDay,
    ThreeDay,
    OneWeek,
    OneMonth,
    ThreeMonth,
    SixMonth,
    OneYear,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Color {
    H660000,
    H990000,
    Hcc0000,
    Hcc3333,
    Hea4c88,
    H993399,
    H663399,
    H333399,
    H0066cc,
    H0099cc,
    H66cccc,
    H77cc33,
    H669900,
    H336600,
    H666600,
    H999900,
    Hcccc33,
    Hffff00,
    Hffcc33,
    Hff9900,
    Hff6600,
    Hcc6633,
    H996633,
    H663300,
    H000000,
    H999999,
    Hcccccc,
    Hffffff,
    H424153,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Query {
    query: String,
    category: Vec<Category>,
    purity: Vec<Purity>,
    sort: Sorting,
    order: Order,
    top_range: TopRange,
    atleast: String,
    resolutions: Vec<String>,
    ratios: Vec<String>,
    colors: Color,
    page: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum QueryOptions {
    Search(Box<Query>),
    Collection(Box<String>),
    Tag(Box<String>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    apikey: String,
    query: QueryOptions,
    seconds: u8,
    // Assuming Scripts takes a pathName to wall.
    script: String,
}
