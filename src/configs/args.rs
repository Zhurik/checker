use clap::Parser;

/// Simple synthetic tester
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to configuration file.
    #[arg(short, long, default_value_t = String::from("./checker.toml"))]
    pub configuration: String,
}
