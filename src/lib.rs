use csv::{ReaderBuilder, Writer};
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::{thread, time};

type MyResult<T> = Result<T, Box<dyn Error>>;

// ===========================================================
#[derive(Serialize, Deserialize)]
struct VikiResponse {
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

// ===========================================================
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WmdaItem {
    douban_id: String,
    douban_rating: String,
    data: Vec<Data>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Data {
    name: String,
}

// ===========================================================
const COUNTRIES: [&str; 5] = ["kr", "cn", "jp", "tw", "th"];
const V_ROOT_URL: &str = "https://api.viki.io/v4/containers.json?page=";
const PARAMETERS: &str = "&per_page=50&with_paging=false&order=desc&sort=views_recent&licensed=true&app=100000a&origin_country=";
const VIKI_FILE: &str = "./result.csv";
const WMDA_FILE: &str = "./result2.csv";
const W_ROOT_URL: &str = "https://api.wmdb.tv/api/v1/movie/search?q=";
const WMDB_CALL_INTERVAL: u64 = 31; // server side 30sec break restriction;
const ZH_NAME_INDEX: usize = 1;

pub fn create_csv_from_viki() -> MyResult<()> {
    // let mut csv_data = String::from(HEADER.to_vec().join(","));
    let mut csv_data = String::new();

    for country in COUNTRIES.iter() {
        let mut page = 0;
        loop {
            println!("{} - page: {}", country, page);

            let request_url = format!("{}{}{}{}", V_ROOT_URL, page, PARAMETERS, country);
            let resp = http_get(request_url)?;
            let data: VikiResponse = serde_json::from_str(&resp)?;

            csv_data.push_str(&fetch_data(&data.response, country));
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

pub fn create_csv_from_wmda() -> Result<(), Box<dyn Error>> {
    let num_processed_lines = match File::open(WMDA_FILE) {
        Ok(f) => BufReader::new(f).lines().count(),
        _ => 0,
    };

    println!("Already proceeded: {}", num_processed_lines);

    let reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(VIKI_FILE)?;
    let lines_to_process: Vec<_> = reader
        .into_records()
        .enumerate()
        .filter(|&(i, _)| i >= num_processed_lines)
        .map(|(_, v)| v)
        .collect();

    println!("size: {}", &lines_to_process.len());

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(WMDA_FILE)?;
    let mut writer = Writer::from_writer(file);

    for line in lines_to_process {
        let mut record = line?;

        match record.get(ZH_NAME_INDEX) {
            Some(name) if !name.is_empty() => {
                println!("call wmdb: {}", name);
                let response = call_wmdb(&name)?;
                println!("response: {:?}", response);
                record.push_field(&response.0);
                record.push_field(&response.1);
                println!("---> {}", record.iter().count());
                writer.write_record(record.iter())?;
                writer.flush()?;
                println!("Sleep...{} secs", WMDB_CALL_INTERVAL);
                thread::sleep(time::Duration::from_secs(WMDB_CALL_INTERVAL));
            }
            _ => {
                println!("no ch_name");
                record.push_field("N/A");
                record.push_field("N/A");
                println!("---> {}", record.iter().count());
                writer.write_record(record.iter())?;
                writer.flush()?;
            }
        }
    }

    Ok(())
}

fn call_wmdb(name: &str) -> MyResult<(String, String)> {
    let request_url = format!("{}{}", W_ROOT_URL, name);
    let resp = http_get(request_url)?;
    let items: Vec<WmdaItem> = match serde_json::from_str(&resp) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Wmda response: '{}'", resp);
            return Err(Box::new(e));
        }
    };

    for item in items {
        if item.data[0].name == name {
            return Ok((item.douban_id, item.douban_rating));
        }
    }

    Ok(("N/A".to_string(), "N/A".to_string()))
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

fn http_get(url: String) -> MyResult<String> {
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
        let result: VikiResponse = serde_json::from_str(&json_string)?;

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
