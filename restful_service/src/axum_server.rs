use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use r2d2::Pool;
use redis::Client;
use sea_orm::DatabaseConnection;
use tracing::{info, warn};

pub async fn start_server(
    pg_conn: DatabaseConnection,
    redis_pool: Pool<Client>,
) -> anyhow::Result<()> {
    let app_state = AppState {
        pg_conn,
        redis_pool,
    };
    let app = Router::new()
        .route("/", get(handler))
        .layer(
            tower::ServiceBuilder::new()
                .layer(middleware::from_fn(auth)) // step 1
                .layer(middleware::from_fn(check_req)), // step 2
        )
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2023").await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pg_conn: DatabaseConnection,
    redis_pool: Pool<Client>,
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
async fn handler(app_state: State<AppState>) -> Response<String> {
    info!("handler");
    let _ = app_state.pg_conn.ping().await;
    let _ = app_state.redis_pool.get();
    Response::new("hello world".to_string())
}

// #[instrument]
async fn check_req(req: Request, next: Next) -> Result<Response, StatusCode> {
    info!("check_req");
    Ok(next.run(req).await)
}
