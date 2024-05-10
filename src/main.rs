use std::sync::{Arc, Mutex, OnceLock};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use rust_bert::{pipelines::summarization::SummarizationModel, RustBertError};
use tokio::task::JoinError;

#[derive(Debug)]
enum JoinBert {
    Join(JoinError),
    Bert(RustBertError),
}

impl From<JoinError> for JoinBert {
    fn from(value: JoinError) -> Self {
        JoinBert::Join(value)
    }
}

impl From<RustBertError> for JoinBert {
    fn from(value: RustBertError) -> Self {
        JoinBert::Bert(value)
    }
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // GET help text
        .route("/help", get(help))
        // GET help text to root as well why not
        // .route("/", get(help))
        // POST summary server
        .route("/summarize", post(summarize));

    let addr = "127.0.0.1:7878";
    tracing::info!("listening on {}", addr);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    tracing::info!("goodbye!");
}

// TODO add some instructions to /help and maybe root too
async fn help() -> &'static str {
    "make a POST request to /summarize endpoint with text body to get the summarized content."
}

async fn summarize(body: String) -> impl IntoResponse {
    tracing::info!("body: {}", body);
    match summarize_text(body).await {
        Ok(summary) => {
            tracing::info!("summary: {}", summary);
            return summary.into_response();
        }
        Err(err) => match err {
            JoinBert::Join(err) => {
                // if it is tokio that's yelling, then we messed up (probably)
                let err_message = format!("error while asyncing some awaits: {:?}", err);
                tracing::error!(err_message);

                return (StatusCode::INTERNAL_SERVER_ERROR, err_message).into_response();
            }
            JoinBert::Bert(err) => {
                // if it is bert yelling then they probably sent some bad text (probably)
                let err_message = format!("error summarizing text: {:?}", err);
                tracing::error!(err_message);

                return (StatusCode::BAD_REQUEST, err_message).into_response();
            }
        },
    }
}

async fn summarize_text(text: String) -> Result<String, JoinBert> {
    let sum_model = tokio::task::block_in_place(|| summary_model().lock().unwrap());

    let output = sum_model.summarize(&[text]).map_err(|err| {
        println!("error while running the model: {:?}", err);
        JoinBert::Bert(err)
    })?;

    Ok(output[0].clone())
}

// it's like caches the model so that it's only initialized once and then the model is returned after those calls
fn summary_model() -> &'static Arc<Mutex<SummarizationModel>> {
    static MODEL: OnceLock<Arc<Mutex<SummarizationModel>>> = OnceLock::new();

    MODEL.get_or_init(|| {
        Arc::new(Mutex::new(
            SummarizationModel::new(Default::default()).unwrap(),
        ))
    })
}

async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();
    #[cfg(unix)]
    let terminate = signal::unix::signal(signal::unix::SignalKind::terminate());

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
