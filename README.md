# Samurai
<p align="center">
<a href="https://github.com/Ardnys/repo_name">
<img  src="images/logo.png" alt="Logo" width="80" height="80">
</a>
</p>

A samurai is a server that summarizes given text. This is the first samurai implementation that uses locally deployed NLP model.
Originally a part of [CFS]() but it can be used for any summarization task.

## What makes a Samurai?
Making a custom samurai is simple.  But it must conform to following attributes to work with [CFS]():
- It must listen to port 7878 for requests.
- It must listen `/summarize` endpoint for POST requests that accept plain text and return the summarized text as plain text.
- It's strongly recommended to have a `/shutdown` GET endpoint to for graceful shutdown. 
- It's preferred to have a `/help` GET endpoint for a helpful usage example.

## Installation Requirements
- make sure Rust and Cargo are installed
- before running the rust-bert model, you need to install **the correct version of** Libtorch. See manually installed dependencies and relevant links below for this specific model and documentation.

## Usage
Use an HTTP request tool to test the samurai. \
`curl -X POST -d "<text to summarize>" http://localhost:7878/summarize` 

## Built with
- [rust-bert](https://github.com/guillaume-be/rust-bert) for the summary model
- [axum](https://github.com/tokio-rs/axum) for the server

## Manually installed dependencies
- Libtorch 2.1.0 [download here](https://pytorch.org/get-started/locally/). correct version is super important.

## Relevant links and resources
- rust-bert for the NLP model: https://github.com/guillaume-be/rust-bert?tab=readme-ov-file
- axum repo for the server: https://github.com/tokio-rs/axum
- axum docs: https://docs.rs/axum/latest/axum/index.html
