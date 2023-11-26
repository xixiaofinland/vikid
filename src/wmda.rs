use crate::*;
use csv::{ReaderBuilder, Writer};
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::{thread, time};

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
pub fn create_csv() -> MyResult<()> {
    let num_processed_lines = match File::open(WMDA_FILE) {
        Ok(f) => BufReader::new(f).lines().count(),
        _ => 0,
    };

    println!("Already proceeded: {}\nContinue...\n", num_processed_lines);

    let reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(VIKI_FILE)?;
    let lines_to_process: Vec<_> = reader
        .into_records()
        .enumerate()
        .filter(|&(i, _)| i >= num_processed_lines)
        .map(|(_, v)| v)
        .collect();

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
                let response = call_endpoint(&name)?;
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

fn call_endpoint(name: &str) -> MyResult<(String, String)> {
    let request_url = format!("{}{}", W_ROOT_URL, name);
    let resp = crate::http_get(request_url)?;
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
