use clap::Parser;

mod lexer;
mod token;
mod token_type;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path of the script file to run
    file: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.file)
}
