use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Repl, Result};
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

fn help<T>(args: ArgMatches, _context: &mut T) -> Result<Option<std::string::String>> {
    if let Some(command) = args.get_one::<String>("command") {
        match command.as_str() {
            "help" => {
                println!("Usage: help [command]");
                println!("Show this message or get help for a specific command.");
            }
            _ => {
                println!("No help available for command '{}'", command);
            }
        }
    }
    else {
        println!("Usage:");
        println!("  help [command] - Show this message or get help for a specific command");
    }
    Ok(None)
}

fn main() -> Result<()> {
    let mut repl = Repl::new(())
        .with_name("SpotifyClassical")
        .with_version("v0.0.1")
        .with_description("Better classical music discovery on Spotify")
        .with_banner("Welcome to SpotifyClassical! Type 'help' to see available commands.")
        .with_command(
            Command::new("help")
                .arg(Arg::new("command").help("The command to get help for").required(false)),
            help
        );
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