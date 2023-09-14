#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Category {
    General,
    Anime,
    People,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

impl Default for Sorting {
    fn default() -> Self {
        Self::DateAdded
    }
}

impl Sorting {
    pub fn to_str(&self) -> &str {
        match self {
            Sorting::DateAdded => "date_added",
            Sorting::Relevance => "relevance",
            Sorting::Random => "random",
            Sorting::Views => "views",
            Sorting::Favorites => "favorites",
            Sorting::Toplist => "toplist",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Order {
    Desc,
    Asc,
}

impl Default for Order {
    fn default() -> Self {
        Self::Desc
    }
}

impl Order {
    pub fn to_str(&self) -> &str {
        match self {
            Order::Desc => "desc",
            Order::Asc => "asc",
        }
    }
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

impl Default for TopRange {
    fn default() -> Self {
        Self::OneMonth
    }
}

impl TopRange {
    pub fn to_str(&self) -> &str {
        match self {
            TopRange::OneDay => "1d",
            TopRange::ThreeDay => "3d",
            TopRange::OneWeek => "1w",
            TopRange::OneMonth => "1M",
            TopRange::ThreeMonth => "3M",
            TopRange::SixMonth => "6M",
            TopRange::OneYear => "1y",
        }
    }
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

impl Color {
    pub fn to_str(&self) -> &str {
        stringify!(self)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Filter {
    categories: Option<Vec<Category>>,
    purity: Option<Vec<Purity>>,
    sorting: Option<Sorting>,
    order: Option<Order>,
    top_range: Option<TopRange>,
    atleast: Option<String>,
    resolutions: Option<Vec<String>>,
    ratios: Option<Vec<String>>,
    colors: Option<Vec<Color>>,
    page: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Query {
    Search(String),
    Collection { username: String, id: String },
    ID(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    apikey: Option<String>,
    query: Query,
    filters: Filter,
    seconds: Option<u32>,
    // Assuming Scripts takes a pathName to wall.
    script: String,
}

impl AppConfig {
    pub fn geturl(&self) -> String {
        format!(
            "{}?apikey={}&{}",
            self.query.tourl(),
            self.apikey.as_ref().unwrap_or(&String::new()),
            self.filters.tourl()
        )
    }

    pub fn time(&self) -> u32 {
        if let Some(s) = self.seconds {
            s
        } else {
            0
        }
    }

    pub fn set_wall(&self, path: PathBuf) -> std::io::Result<Output> {
        Command::new(format!("{} {}", self.script, path.to_string_lossy())).output()
    }
}

impl Query {
    pub fn tourl(&self) -> String {
        match self {
            Query::ID(id) => format!("https://wallhaven.cc/api/v1/w/{}", id),
            Query::Search(search) => {
                format!("https://wallhaven.cc/api/v1/search?{}", search)
            }
            Query::Collection { username, id } => format!(
                "https://wallhaven.cc/api/v1/collections/{}/{}",
                &username, &id
            ),
        }
    }
}

impl Filter {
    pub fn tourl(&self) -> String {
        format!("categories={}&purity={}&sorting={}&order={}&topRange={}&atleast={}&resolutions={}&ratios={}&colors={}&page={}",
                self.category_str(),
                self.purity_str(),
                self.sorting.as_ref().unwrap_or(&Sorting::DateAdded).to_str(),
                self.order.as_ref().unwrap_or(&Order::Desc).to_str(),
                self.top_range.as_ref().unwrap_or(&TopRange::OneMonth).to_str(),
                self.atleast.as_ref().unwrap_or(&String::from("")),
                self.resolutions_str(),
                self.ratios_str(),
                self.color_str(),
                self.page.unwrap_or_default()
        )
    }

    pub fn category_str(&self) -> String {
        let mut s = String::with_capacity(3);
        if let Some(c) = &self.categories {
            s.push(contains10(&c, Category::General));
            s.push(contains10(&c, Category::Anime));
            s.push(contains10(&c, Category::People));
        } else {
            s = String::from("111")
        }
        s
    }

    pub fn purity_str(&self) -> String {
        let mut s = String::with_capacity(3);
        if let Some(p) = &self.purity {
            s.push(contains10(&p, Purity::SFW));
            s.push(contains10(&p, Purity::Sketchy));
            s.push(contains10(&p, Purity::NSFW));
        } else {
            s = String::from("100")
        }
        s
    }

    pub fn resolutions_str(&self) -> String {
        (self.resolutions.as_ref().unwrap_or(&vec![])).join(",")
    }

    pub fn ratios_str(&self) -> String {
        (self.ratios.as_ref().unwrap_or(&vec![])).join(",")
    }

    pub fn color_str(&self) -> String {
        self.colors
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|c| c.to_str())
            .collect::<String>()
    }
}

fn contains10<T: PartialEq>(v: &Vec<T>, c: T) -> char {
    if v.contains(&c) {
        '1'
    } else {
        '0'
    }
}

pub fn load_config(path: &PathBuf) -> AppConfig {
    let mut file = File::open(path).expect("catch later file open");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("catch later file reading to string");
    ron::from_str::<AppConfig>(&s).expect("catch later ron_file to AppConfig")
}

#[test]
fn test_printexample() {
    let conf = load_config(&PathBuf::from("example.ron"));
    println!("{:#?}", conf);
}
