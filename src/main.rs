mod repl;

use reedline_repl_rs::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    piece: String,
    #[arg(short, long)]
    composer: String,
    #[arg(short, long)]
    musician: String,
    #[arg(long)]
    conductor: String,
    #[arg(long)]
    orchestra: String,
    #[arg(short, long)]
    year: Option<u64>,
    #[arg(short, long)]
    live: bool
}

fn main() -> Result<()> {
    let mut repl = repl::create_repl()?;
    repl.run()

    // let cli = Cli::parse();

    // println!("piece: {:?}", cli.piece);
    // println!("composer: {:?}", cli.composer);
    // println!("musician: {:?}", cli.musician);
    // println!("conductor: {:?}", cli.conductor);
    // println!("orchestra: {:?}", cli.orchestra);
    // println!("year: {:?}", cli.year);
    // println!("live: {:?}", cli.live);
}