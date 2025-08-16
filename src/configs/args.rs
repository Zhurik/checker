//! Command-line argument parsing using clap
//!
//! This module defines the command-line interface for the checker application
//! using the clap derive API.

use clap::Parser;

/// Command-line arguments for the checker application
///
/// This struct represents all available command-line options that can be
/// passed to the application.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to configuration file.
    ///
    /// Specifies the path to the TOML configuration file that contains
    /// the Telegram token, chat ID, Docker image name, and other settings.
    #[arg(short, long, default_value_t = String::from("./checker.toml"))]
    pub configuration: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_args() {
        let args = Args::parse_from(["test"]);
        assert_eq!(args.configuration, "./checker.toml");
    }

    #[test]
    fn test_custom_config_path() {
        let args = Args::parse_from(["test", "-c", "/custom/path/config.toml"]);
        assert_eq!(args.configuration, "/custom/path/config.toml");
    }

    #[test]
    fn test_long_config_flag() {
        let args = Args::parse_from(["test", "--configuration", "/another/path/config.toml"]);
        assert_eq!(args.configuration, "/another/path/config.toml");
    }
}
