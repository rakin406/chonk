use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    /// Path of the script file to run
    pub file: Option<String>,
}

impl Cli {
    /// Returns `true` if the `Option` fields are of `None` value.
    pub fn is_empty(&self) -> bool {
        self.file.is_none()
    }
}
