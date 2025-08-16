//! Integration tests for the checker application
//!
//! These tests verify the end-to-end functionality of the application,
//! including configuration loading and command-line argument parsing.

use checker::configs::{args, config};
use clap::Parser;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_config_loading_from_file() {
    // Create a temporary directory for our test config file
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_config.toml");
    
    // Create a test configuration file
    let config_content = r#"
        token = "test_token_123"
        chat_id = 987654321
        image = "alpine:latest"
        people = ["TestUser1", "TestUser2"]
    "#;
    
    fs::write(&file_path, config_content).unwrap();
    
    // Test loading the configuration
    let loaded_config = config::from_file(file_path.to_string_lossy().to_string()).unwrap();
    
    assert_eq!(loaded_config.token, "test_token_123");
    assert_eq!(loaded_config.chat_id, 987654321);
    assert_eq!(loaded_config.image, "alpine:latest");
    assert_eq!(loaded_config.people, vec!["TestUser1", "TestUser2"]);
}

#[test]
fn test_config_file_not_found() {
    let result = config::from_file("/non/existent/path/config.toml".to_string());
    assert!(result.is_err());
}

#[test]
fn test_command_line_args_parsing() {
    let args = args::Args::try_parse_from(["checker", "-c", "./test_config.toml"]).unwrap();
    assert_eq!(args.configuration, "./test_config.toml");
    
    let args = args::Args::try_parse_from(["checker", "--configuration", "/custom/config.toml"]).unwrap();
    assert_eq!(args.configuration, "/custom/config.toml");
}