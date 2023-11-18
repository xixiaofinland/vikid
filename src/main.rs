use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct VikiResponse {
    more: bool,
    response: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    titles: Title,
    subtitle_completions: Subtitle,
    url: Url,
    review_stats: Review,
    clips: Option<Clip>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Title {
    en: String,
    zh: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Subtitle {
    #[serde(default)]
    fi: Option<Number>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Url {
    web: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Review {
    average_rating: Number,
    count: Number,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clip {
    count: Number,
}

const COUNTRIES: [&str; 5] = ["kr", "cn", "jp", "tw", "th"];
const ROOT_URL: &str = "https://api.viki.io/v4/containers.json?page=";
const PARAMETERS: &str = "&per_page=50&with_paging=false&order=desc&sort=views_recent&licensed=true&app=100000a&origin_country=";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut page = 0;
    let mut country = COUNTRIES[0];

    let mut csvData = String::from("title_en,title_zh,url,FI,rate,rateCount,clipsCount\n");
    loop {
        let request_url = format!("{}{}{}{}", ROOT_URL, page, PARAMETERS, country);
        let resp = reqwest::get(request_url)
            .await?
            .json::<VikiResponse>()
            .await?;
        csvData.push_str(&fetch_data(&resp.response));

        match resp.more {
            true => break, //page += 1,
            false => break,
        }
    }

    fs::write("./result.csv", csvData)?;
    Ok(())
}

fn fetch_data(items: &Vec<Item>) -> String {
    let mut data = String::new();
    for item in items.iter() {
        data.push_str(&parse_data(item));
    }
    data
}

fn parse_data(item: &Item) -> String {
    let en = &item.titles.en;
    let zh = &item.titles.zh;
    let url = &item.url.web;
    let fi: Number = match &item.subtitle_completions.fi {
        Some(n) => n.clone(),
        None => 0.into(),
    };
    let rate = &item.review_stats.average_rating;
    let rate_count = &item.review_stats.count;
    let clips_count: Number = match &item.clips {
        Some(c) => c.count.clone(),
        None => 0.into(),
    };

    format!(
        "{},{},{},{},{},{},{}\n",
        en, zh, url, fi, rate, rate_count, clips_count
    )
}
