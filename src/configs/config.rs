//! Configuration file parsing
//!
//! This module handles parsing of the TOML configuration file that contains
//! the application settings.

use std;

use serde::Deserialize;

/// Application configuration
///
/// This struct holds all configuration values needed for the application
/// to operate, loaded from a TOML configuration file.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// Telegram bot token
    ///
    /// The API token for the Telegram bot used to send messages.
    pub token: String,

    /// Telegram chat ID
    ///
    /// The ID of the chat where performance results and error notifications
    /// will be sent.
    pub chat_id: i64,

    /// Docker image to test
    ///
    /// The name of the Docker image to pull for performance testing.
    pub image: String,

    /// People to mention in notifications
    ///
    /// A list of people to mention in error notifications.
    pub people: Vec<String>,
}

/// Load configuration from a TOML file
///
/// Reads and parses a TOML configuration file from the specified path.
///
/// # Arguments
///
/// * `path` - Path to the TOML configuration file
///
/// # Returns
///
/// Returns `Ok(Config)` with the parsed configuration, or an error if
/// the file cannot be read or parsed.
pub fn from_file(path: String) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = match std::fs::read_to_string(path) {
        Ok(content) => content.to_owned(),
        Err(e) => return Err(Box::new(e)),
    };

    let config: Config = match toml::from_str(&contents[..]) {
        Ok(config) => config,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config_parsing() {
        let toml_content = r#"
            token = "test_token"
            chat_id = 123456789
            image = "nginx:latest"
            people = ["Alice", "Bob"]
        "#;

        let config: Config = toml::from_str(toml_content).expect("Failed to parse valid TOML");
        
        assert_eq!(config.token, "test_token");
        assert_eq!(config.chat_id, 123456789);
        assert_eq!(config.image, "nginx:latest");
        assert_eq!(config.people, vec!["Alice", "Bob"]);
    }

    #[test]
    fn test_config_with_empty_people_list() {
        let toml_content = r#"
            token = "test_token"
            chat_id = 123456789
            image = "nginx:latest"
            people = []
        "#;

        let config: Config = toml::from_str(toml_content).expect("Failed to parse valid TOML");
        
        assert_eq!(config.token, "test_token");
        assert_eq!(config.chat_id, 123456789);
        assert_eq!(config.image, "nginx:latest");
        assert_eq!(config.people, Vec::<String>::new());
    }

    #[test]
    fn test_invalid_config_missing_fields() {
        let toml_content = r#"
            token = "test_token"
            # Missing chat_id, image, and people fields
        "#;

        let result: Result<Config, toml::de::Error> = toml::from_str(toml_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_with_special_characters() {
        let toml_content = r#"
            token = "test_token_with_special_chars_!@#$%^&*()"
            chat_id = -123456789
            image = "my-repo/my-app:v1.2.3-beta"
            people = ["Алиса", "Боб", "测试用户"]
        "#;

        let config: Config = toml::from_str(toml_content).expect("Failed to parse valid TOML");
        
        assert_eq!(config.token, "test_token_with_special_chars_!@#$%^&*()");
        assert_eq!(config.chat_id, -123456789);
        assert_eq!(config.image, "my-repo/my-app:v1.2.3-beta");
        assert_eq!(config.people, vec!["Алиса", "Боб", "测试用户"]);
    }
}
