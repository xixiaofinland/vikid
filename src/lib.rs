#![allow(dead_code, unused, unused_imports)]

use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::error::Error;
use std::fs;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize)]
pub struct VikiResponse {
    more: bool,
    response: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
struct Item {
    titles: Title,
    subtitle_completions: Subtitle,
    url: Url,
    review_stats: Review,
    clips: Option<Clip>,
    created_at: String,
}

#[derive(Serialize, Deserialize)]
struct Title {
    en: String,
    #[serde(default)]
    zh: String,
}

#[derive(Serialize, Deserialize)]
struct Subtitle {
    fi: Option<Number>,
}

#[derive(Serialize, Deserialize)]
struct Url {
    web: String,
}

#[derive(Serialize, Deserialize)]
struct Review {
    average_rating: Number,
    count: Number,
}

#[derive(Serialize, Deserialize)]
struct Clip {
    count: Number,
}

const COUNTRIES: [&str; 5] = ["kr", "cn", "jp", "tw", "th"];
const ROOT_URL: &str = "https://api.viki.io/v4/containers.json?page=";
const PARAMETERS: &str = "&per_page=50&with_paging=false&order=desc&sort=views_recent&licensed=true&app=100000a&origin_country=";

fn main() -> Result<(), Box<dyn Error>> {
    let mut csv_data =
        String::from("title_en,title_zh,url,fi,rate,rate_count,clips_count,created_at,country\n");

    for country in COUNTRIES.iter() {
        let mut page = 0;
        loop {
            println!("{} - page: {}", country, page);
            let request_url = format!("{}{}{}{}", ROOT_URL, page, PARAMETERS, country);
            let resp = reqwest::blocking::get(request_url)?.json::<VikiResponse>()?;

            csv_data.push_str(&fetch_data(&resp.response, country));

            match resp.more {
                true => page += 1,
                false => {
                    break;
                }
            }
        }
    }

    fs::write("./result.csv", csv_data)?;
    Ok(())
}

fn fetch_data(items: &Vec<Item>, country: &str) -> String {
    let mut data = String::new();
    for item in items.iter() {
        data.push_str(&parse_data(item, country));
    }
    data
}

fn parse_data(item: &Item, country: &str) -> String {
    let en = &item.titles.en;
    let zh = &item.titles.zh;
    let url = &item.url.web;
    let fi: Number = match &item.subtitle_completions.fi {
        Some(n) => n.clone(),
        None => return "".into(),
    };
    let rate = &item.review_stats.average_rating;
    let rate_count = &item.review_stats.count;
    let clips_count: Number = match &item.clips {
        Some(c) => c.count.clone(),
        None => 0.into(),
    };
    let created_at = &item.created_at;

    format!(
        "\"{}\",\"{}\",{},{},{},{},{},{},{}\n",
        en, zh, url, fi, rate, rate_count, clips_count, created_at, country
    )
}

fn get_http(url: String) -> MyResult<String> {
    Ok(reqwest::blocking::get(url)?.text()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    // use httpmock::prelude::*;
    type TestResult = Result<(), Box<dyn std::error::Error>>;

    const viki_res: &str = "tests/viki_response.txt";

    #[test]
    fn can_serielize() -> TestResult {
        let json_string = fs::read_to_string(viki_res)?;
        let result = serde_json::from_str::<VikiResponse>(&json_string)?;

        assert_eq!(result.more, true);
        assert_eq!(result.response.len(), 1);

        Ok(())
    }

    // #[test]
    // fn can_get_http() -> TestResult {
    //     let server = MockServer::start();
    //     let viki_mock = server.mock(|when, then| {
    //         when.method(GET);
    //         then.status(200).body("hello world!");
    //     });
    //
    //     let response = get_http(server.url("/"))?;
    //
    //     viki_mock.assert();
    //     assert_eq!(response, "hello world!");
    //     Ok(())
    // }
}
