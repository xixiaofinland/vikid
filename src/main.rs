use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Serialize, Deserialize, Debug)]
struct VikiResponse {
    more: bool,
    response: Vec<Element>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Element {
    // titles: Vec<Title>,
    // subtitle_completions: Subtitle,
    // url: Url,
    // review_stats: Review,
    clips: Option<Clip>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Title {
    en: String,
    zh: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Subtitle {
    fi: Option<Number>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Url {
    web: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Review {
    average_rating: String,
    count: Number,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clip {
    count: Number,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let countries = ["kr", "cn", "jp", "tw", "th"];
    let root_url = "https://api.viki.io/v4/containers.json?page=";
    let parameters = "&per_page=50&with_paging=false&order=desc&sort=views_recent&licensed=true&app=100000a&origin_country=";

    let request_url = format!("{}{}{}{}", root_url, 0, parameters, "th");
    println!("{}", request_url);

    let resp = reqwest::get(request_url)
        .await?
        .json::<VikiResponse>()
        .await?;

    println!("{:#?}", &resp);
    Ok(())
}
