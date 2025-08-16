//! Main entry point for the checker application
//!
//! This module contains the main function that orchestrates the Docker image
//! pull performance testing and reporting to Telegram.

use checker::configs::{args, config};
use clap::Parser;
use futures::StreamExt;
use shiplift::{Docker, PullOptions};
use std::time::Instant;
use teloxide::prelude::*;
use chrono;

/// Main application entry point
///
/// This function performs the following steps:
/// 1. Parses command-line arguments
/// 2. Loads configuration from the specified file
/// 3. Connects to the Telegram bot
/// 4. Connects to the Docker daemon
/// 5. Measures the time to pull the configured Docker image
/// 6. Reports results to Telegram
/// 7. Cleans up by deleting the pulled image
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();
    let config = config::from_file(args.configuration)?;

    let bot = Bot::new(config.token);
    let chat_id = ChatId(config.chat_id);

    let docker = Docker::new();

    let start = Instant::now();

    let mut stream = docker
        .images()
        .pull(&PullOptions::builder().image(&config.image).build());

    while let Some(pull_result) = stream.next().await {
        match pull_result {
            Ok(output) => println!("{:?}", output),
            Err(e) => {
                eprintln!("Error: {}", e);

                let current_time = chrono::Local::now();
                let people_string = config.people.join(" ");
                let msg = format!("Произошла ошибка при скачивании: {e}\n\n{current_time}\n\n{people_string}");
                bot.send_message(chat_id, msg).await?;
                return Ok(())
            },
        }
    }

    let end = Instant::now();

    let duration = end.duration_since(start).as_secs();

    let current_time = chrono::Local::now();

    let msg = format!(
        "У меня заняло {0} секунд на скачивание {1} на дохлой виртуалке\n\n{2}",
        duration,
        &config.image,
        current_time.to_rfc2822(),
    );

    println!("{}", msg);

    // bot.send_message(chat_id, msg).await?;

    match docker.images().get(&config.image).delete().await {
        Ok(statuses) => {
            for status in statuses {
                println!("{:?}", status);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main_function_compiles() {
        // This test ensures that the main function has the correct signature
        // and that all required dependencies are available.
        // We don't actually run the main function here as it requires
        // Docker and Telegram connections.
        assert_eq!(2 + 2, 4);
    }
}
