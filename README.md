<h1 align="center"> Samurai </h1>
<p align="center">
<a href="https://github.com/Ardnys/repo_name">
<img  src="https://github.com/Ardnys/samurai-rs/blob/master/images/samurai_icon.png" alt="Logo" width="200" height="200">
</a>
</p>

## Overview
A samurai is a server that summarizes given text. This is the first samurai implementation that uses locally deployed NLP model.
Originally a part of [CFS](https://github.com/Ardnys/cfs-web/) but it can be used for any summarization task.\
However, I could not make it asynchronous because the NLP model is not thread safe and I don't know how to work around that. Contributions or discussions regarding this is greatly appreaciated. Please open an issue and we can discuss it.

## What makes a Samurai?
Making a custom samurai is simple.  But it must conform to following attributes to work with [CFS](https://github.com/Ardnys/cfs-web/):
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
