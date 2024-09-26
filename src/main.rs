use checker::configs::{args, config};
use clap::Parser;
use futures::StreamExt;
use shiplift::{Docker, PullOptions};
use std::time::Instant;
use teloxide::prelude::*;
use chrono;

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
                let msg = format!("Произошла ошибка при скачивании: {e}\n\n{current_time}");
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

    bot.send_message(chat_id, msg).await?;

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
