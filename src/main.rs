use clap::Parser;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arg {
    #[arg(
        short('v'),
        long("viki"),
        conflicts_with("douban_only"),
        help("Retrieve only basic info from viki")
    )]
    viki_only: bool,

    #[arg(
        short('d'),
        long("douban"),
        conflicts_with("viki_only"),
        help("Retrieve only extra info from wmda (based on existing viki result)")
    )]
    wmda_only: bool,
}

fn main() {
    if let Err(e) = run(Arg::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub fn run(arg: Arg) -> MyResult<()> {
    if arg.viki_only == true {
        println!("VIKI data pulling..");
        vikid::create_csv_from_viki()?;
    } else if arg.wmda_only == true {
        println!("WMDA data pulling...");
        vikid::create_csv_from_wmda()?;
    } else {
        println!("VIKI data pulling..");
        vikid::create_csv_from_viki()?;
        println!("WMDA data pulling...");
        vikid::create_csv_from_wmda()?;
    }

    Ok(())
}
