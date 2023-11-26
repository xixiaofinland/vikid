pub mod viki;
pub mod wmda;

pub type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

// ===========================================================
pub const COUNTRIES: [&str; 5] = ["kr", "cn", "jp", "tw", "th"];
pub const V_ROOT_URL: &str = "https://api.viki.io/v4/containers.json?page=";
pub const PARAMETERS: &str = "&per_page=50&with_paging=false&order=desc&sort=views_recent&licensed=true&app=100000a&origin_country=";
pub const VIKI_FILE: &str = "./viki.csv";
pub const WMDA_FILE: &str = "./viki_wmda.csv";
pub const W_ROOT_URL: &str = "https://api.wmdb.tv/api/v1/movie/search?q=";
pub const WMDB_CALL_INTERVAL: u64 = 31; // server side has 30sec interval calling restriction;
pub const ZH_NAME_INDEX: usize = 1; // assume chinese name is saved in the 2nd column of VIKI_FILE;

pub fn http_get(url: String) -> MyResult<String> {
    Ok(reqwest::blocking::get(url)?.text()?)
}
