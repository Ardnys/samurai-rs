# First Samurai of Course Feedback System
First samurai implementation that uses locally deployed NLP model. I couldn't do it async though.

## Installation Requirements
- see manually installed dependencies and relevant links below
- also make sure Rust and Cargo are installed

## Usage
i used `curl` a lot to test this one. something like: \
`curl -X POST -d "<text to summarize>" http://localhost:7878/summarize` \
and that's about it.

## Built upon
- axum for the server
- rust-bert for the summary model

## Manually installed dependencies
- Libtorch 2.1.0 [download here](https://pytorch.org/get-started/locally/). correct version is super important.

## Relevant links and resources
- https://github.com/tokio-rs/axum
- https://docs.rs/axum/latest/axum/index.html
- https://github.com/guillaume-be/rust-bert?tab=readme-ov-file
