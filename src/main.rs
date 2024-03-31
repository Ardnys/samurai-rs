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
        // `GET /` goes to `root`
        .route("/", get(root))
        // POST echo server
        .route("/echo", post(echo));

    let addr = "127.0.0.1:3000";
    tracing::info!("listening on {}", addr);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    tracing::info!("goodbye!");
}

// TODO add some instructions to root as well as /help
async fn root() -> &'static str {
    "Hello, World!"
}

// TODO maybe filter the incoming data. They could send JSON or anything. Feels kinda dangerous.
// I don't think I needed JSON parsing or anything but maybe we do. Something to think about at least.
async fn echo(body: String) -> impl IntoResponse {
    tracing::info!("body: {}", body);
    match summarize_text(body).await {
        Ok(summary) => {
            tracing::info!("summarized text: {}", summary);

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

// that error handling didn't take a few hours or anything
async fn summarize_text(text: String) -> Result<String, JoinBert> {
    let sum_model_result =
        tokio::task::spawn_blocking(|| SummarizationModel::new(Default::default())).await;

    let sum_model = match sum_model_result {
        Ok(Ok(model)) => model,
        Ok(Err(err)) => return Err(JoinBert::Bert(err)),
        Err(join_err) => return Err(JoinBert::Join(join_err)),
    };

    let output = sum_model.summarize(&[text]).map_err(|err| {
        println!("error while running the model: {:?}", err);
        JoinBert::Bert(err)
    })?;

    Ok(output[0].clone())
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
