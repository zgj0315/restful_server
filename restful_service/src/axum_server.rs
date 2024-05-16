use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use tracing::{info, warn};

pub async fn start_server() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(handler)).layer(
        tower::ServiceBuilder::new()
            .layer(middleware::from_fn(auth)) // step 1
            .layer(middleware::from_fn(check_req)), // step 2
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2023").await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        warn!("ctrl_c is running");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
        warn!("kill is running");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

// #[instrument]
async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    info!("auth");

    Ok(next.run(req).await)
}

// #[instrument]
async fn handler() -> Response<String> {
    info!("handler");
    Response::new("hello world".to_string())
}

// #[instrument]
async fn check_req(req: Request, next: Next) -> Result<Response, StatusCode> {
    info!("check_req");
    Ok(next.run(req).await)
}
