use clap::Parser;

mod lexer;
mod token;
mod token_type;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path of the script file to run
    #[arg(required(false))]
    file: String,
}

fn main() {
    let args = Args::parse();
    println!("The filename is {}", args.file);
}
