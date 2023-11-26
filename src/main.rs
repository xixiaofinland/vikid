use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arg {
    #[arg(
        short('v'),
        long("viki"),
        conflicts_with("wmda_only"),
        help("Retrieve only basic info from viki")
    )]
    viki_only: bool,

    #[arg(
        short('w'),
        long("wmda"),
        conflicts_with("viki_only"),
        help(
            "Assume viki csv was created, retrieve only data from wmda (i.e. douban id, douban rating)"
        )
    )]
    wmda_only: bool,
}

fn main() {
    if let Err(e) = run(Arg::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub fn run(arg: Arg) -> vikid::MyResult<()> {
    if arg.viki_only == true {
        println!("VIKI data pulling..");
        vikid::viki::create_csv()?;
    } else if arg.wmda_only == true {
        println!("WMDA data pulling...");
        vikid::wmda::create_csv()?;
    } else {
        println!("VIKI data pulling..");
        vikid::viki::create_csv()?;
        println!("WMDA data pulling...");
        vikid::wmda::create_csv()?;
    }

    Ok(())
}
