# Checker

A synthetic testing tool that pulls Docker images and reports performance metrics to a Telegram chat.

## Overview

Checker is a Rust application that:

1. Pulls a specified Docker image

2. Measures the time it takes to download the image

3. Reports the results to a Telegram chat

4. Cleans up by deleting the downloaded image

This tool is designed for synthetic testing of Docker registry performance, particularly in environments where performance may be degraded.

## Features

- Measures Docker image pull performance

- Sends results to Telegram chat

- Configurable through TOML configuration file

- Error reporting with notifications

- Automatic cleanup of downloaded images

- Cross-compilation support with static SSL feature

## Prerequisites

- Rust toolchain

- Docker daemon running

- Telegram bot token

- Access to Docker images to test

## Installation

```bash
cargo build --release
```

For systems without OpenSSL installed or for cross-compilation:

```bash
cargo build --release --features static_ssl
```

## Configuration

Create a `checker.toml` file with the following structure:

```toml
token = "YOUR_TELEGRAM_BOT_TOKEN"
chat_id = 123456789
image = "nginx:latest"
people = ["@Alice", "@Bob", "@Charlie"]
```

Configuration options:

- `token`: Your Telegram bot token

- `chat_id`: The ID of the chat where results will be sent

- `image`: The Docker image to pull for testing

- `people`: A list of people to mention in error notifications

## Usage

```bash
./target/release/checker [OPTIONS]
```

Options:
- `-c, --configuration <CONFIGURATION>`: Path to configuration file [default: ./checker.toml]

Example:
```bash
./target/release/checker -c ./config/checker.toml
```

## How it works

1. The application reads the configuration file

2. Connects to the Docker daemon

3. Starts pulling the configured Docker image

4. Measures the time taken to pull the image

5. Sends the timing results to the configured Telegram chat

6. If an error occurs during the pull, sends an error notification

7. Deletes the pulled image to clean up
