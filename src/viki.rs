use crate::*;
use serde::Deserialize;
use serde_json::Number;
use std::fs;

// ===========================================================
#[derive(Deserialize)]
struct VikiResponse {
    more: bool,
    response: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    titles: Title,
    subtitle_completions: Subtitle,
    url: Url,
    review_stats: Review,
    clips: Option<Clip>,
    created_at: String,
}

#[derive(Deserialize)]
struct Title {
    en: String,
    #[serde(default)]
    zh: String,
}

#[derive(Deserialize)]
struct Subtitle {
    fi: Option<Number>,
}

#[derive(Deserialize)]
struct Url {
    web: String,
}

#[derive(Deserialize)]
struct Review {
    average_rating: Number,
    count: Number,
}

#[derive(Deserialize)]
struct Clip {
    count: Number,
}

// ===========================================================
pub fn create_csv() -> MyResult<()> {
    // let mut csv_data = String::from(HEADER.to_vec().join(","));
    let mut csv_data = String::new();

    for country in COUNTRIES.iter() {
        let mut page = 0;
        loop {
            println!("{} - page: {}", country, page);

            let request_url = format!("{}{}{}{}", V_ROOT_URL, page, PARAMETERS, country);
            let resp = crate::http_get(request_url)?;
            let data: VikiResponse = serde_json::from_str(&resp)?;

            csv_data.push_str(&process_data(&data.response, country));

            match data.more {
                true => page += 1,
                false => {
                    break;
                }
            }
        }
    }

    fs::write(VIKI_FILE, csv_data)?;
    Ok(())
}

fn process_data(items: &Vec<Item>, country: &str) -> String {
    let mut data = String::new();
    for item in items.iter() {
        data.push_str(&filter_data(item, country));
    }
    data
}

fn filter_data(item: &Item, country: &str) -> String {
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
