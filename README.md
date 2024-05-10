listens some port for incoming text and returns a summary using a local llm

# Built upon
- axum for the server
- rust-bert for the summary model

# Manually installed dependencies
- Libtorch 2.1.0 [download here](https://pytorch.org/get-started/locally/). correct version is super important.

# Relevant links and resources
- https://github.com/tokio-rs/axum
- https://docs.rs/axum/latest/axum/index.html
- https://github.com/guillaume-be/rust-bert?tab=readme-ov-file
