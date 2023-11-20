use clap::Parser;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arg {
    #[arg(short('d'), long("douban"))]
    pull_extra: bool,
}
fn main() {
    if let Err(e) = run(Arg::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub fn run(arg: Arg) -> MyResult<()> {
    println!("VIKI data pulling..");
    // vikid::create_csv_from_viki()?;

    if arg.pull_extra == true {
        println!("WMDA data pulling...");
        vikid::create_csv_from_wmda()?;
    }

    Ok(())
}
