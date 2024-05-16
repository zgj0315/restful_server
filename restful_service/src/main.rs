use restful_service::axum_server;

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
    axum_server::start_server().await?;
    Ok(())
}
