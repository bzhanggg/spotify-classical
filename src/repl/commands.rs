use reedline_repl_rs::Result;
use reedline_repl_rs::clap::ArgMatches;

pub fn search<T>(args: ArgMatches, _context: &mut T) -> Result<Option<String>> {
    if let Some(search_toks) = args.get_many::<String>("search-string") {
        let query = search_toks
            .map(|t| t.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        println!("Searching for: {}", query);
    }
    Ok(None)
}
