#[tokio::main]
async fn main() -> anyhow::Result<()> {
    struct LocalTimer;
    impl tracing_subscriber::fmt::time::FormatTime for LocalTimer {
        fn format_time(
            &self,
            w: &mut tracing_subscriber::fmt::format::Writer<'_>,
        ) -> std::fmt::Result {
            write!(w, "{}", chrono::Local::now().format("%m-%d %H:%M:%S.%6f"))
        }
    }

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .event_format(
            tracing_subscriber::fmt::format()
                .with_level(true)
                .with_target(true)
                .with_file(false)
                .with_line_number(true)
                .with_timer(LocalTimer),
        )
        .init();
    tracing::info!("log init success");
    let url =
        "postgres://restful_server_user:restful_server_password@127.0.0.1:5432/restful_server_db";
    let mut opt: sea_orm::ConnectOptions = sea_orm::ConnectOptions::new(url);
    opt.max_connections(20);
    let pg_conn = sea_orm::Database::connect(opt).await?;
    tracing::info!("pg conn init success");
    restful_service::axum_server::start_server(pg_conn).await?;
    Ok(())
}
