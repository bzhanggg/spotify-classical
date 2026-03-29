mod repl;

use reedline_repl_rs::Result;

fn main() -> Result<()> {
    let mut repl = repl::create_repl()?;
    repl.run()
}
