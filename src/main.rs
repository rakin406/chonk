use clap::Parser;

mod internal;
mod repl;
mod runner;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path of the script file to run
    file: Option<String>,
}

fn main() -> rustyline::Result<()> {
    let args = Args::parse();

    if args.file.is_none() {
        let version = env!("CARGO_PKG_VERSION");

        // TODO: Create a template for `help` command.
        println!(
            "\
            Welcome to Chonk {}.\n\
            Type \".help\" for more information.\
            ",
            version
        );
        repl::run_prompt()?;
    } else if let Some(file) = args.file {
        runner::run_file(&file);
    }

    Ok(())
}
