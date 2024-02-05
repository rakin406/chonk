use clap::Parser;
use rustyline::Result;

mod internal;
mod repl;
mod runner;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path of the script file to run
    file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(file) = args.file {
        runner::run_file(&file);
    } else {
        let version = env!("CARGO_PKG_VERSION");

        println!(
            "\
            Welcome to Chonk {}.\n\
            Type \".help\" for more information.\
            ",
            version
        );
        repl::start()?;
    }

    Ok(())
}
