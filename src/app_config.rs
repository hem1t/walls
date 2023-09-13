use serde::{Deserialize, Serialize};

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

impl Sorting {
    pub fn to_string(&self) -> &str {
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

impl Order {
    pub fn to_string(&self) -> &str {
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

impl TopRange {
    pub fn to_string(&self) -> &str {
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
pub struct SearchURL {
    query: String,
    category: Vec<Category>,
    purity: Vec<Purity>,
    sort: Sorting,
    order: Order,
    top_range: TopRange,
    atleast: String,
    resolutions: Vec<String>,
    ratios: Vec<String>,
    colors: Vec<Color>,
    page: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Query {
    Search(Box<SearchURL>),
    Collection {
        username: Box<String>,
        id: Box<String>,
    },
    ID(Box<String>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    apikey: Option<String>,
    username: Option<String>,
    query: Query,
    seconds: Option<u32>,
    // Assuming Scripts takes a pathName to wall.
    script: String,
}

impl Query {
    pub fn tourl(self) -> String {
        match self {
            Query::ID(id) => format!("https://wallhaven.cc/api/v1/w/{}", id),
            Query::Search(search) => {
                format!("https://wallhaven.cc/api/v1/search?{}", search.tourl())
            }
            Query::Collection { username, id } => format!(
                "https://wallhaven.cc/api/v1/collections/{}/{}",
                &username, &id
            ),
        }
    }
}

impl SearchURL {
    pub fn tourl(self) -> String {
        format!("q={}&categories={}&purity={}&sorting={}&order={}&topRange={}&atleast={}&resolutions={}&ratios={}&colors={}&page={}",
                self.query,
                self.category_str(),
                self.purity_str(),
                self.sort.to_string(),
                self.order.to_string(),
                self.top_range.to_string(),
                self.atleast,
                self.resolutions_str(),
                self.ratios_str(),
                self.color_str(),
                self.page
        )
    }

    pub fn category_str(&self) -> String {
        let mut s = String::with_capacity(3);
        let category = |c: Category| {
            if self.category.contains(&c) {
                '1'
            } else {
                '0'
            }
        };
        s.push(category(Category::General));
        s.push(category(Category::Anime));
        s.push(category(Category::People));
        s
    }

    pub fn purity_str(&self) -> String {
        let mut s = String::with_capacity(3);
        let purity = |c: Purity| {
            if self.purity.contains(&c) {
                '1'
            } else {
                '0'
            }
        };
        s.push(purity(Purity::SFW));
        s.push(purity(Purity::Sketchy));
        s.push(purity(Purity::NSFW));
        s
    }

    pub fn resolutions_str(&self) -> String {
        self.resolutions.join(",")
    }

    pub fn ratios_str(&self) -> String {
        self.ratios.join(",")
    }

    pub fn color_str(&self) -> String {
        self.colors.iter().map(|c| c.to_str()).collect::<String>()
    }
}
