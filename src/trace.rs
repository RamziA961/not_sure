pub(crate) fn initialize() {
    use std::fs;
    use tracing_subscriber::{filter, fmt, prelude::*};

    let log_analysis_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("./log.json")
        .unwrap();

    let log_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("./log.txt")
        .unwrap();

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .pretty()
                .with_filter(filter::Targets::new().with_target("not_sure", {
                    #[cfg(debug_assertions)]
                    {
                        filter::LevelFilter::TRACE
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        filter::LevelFilter::INFO
                    }
                })),
        )
        .with(
            fmt::layer()
                .with_ansi(false)
                .with_writer(std::sync::Arc::new(log_file))
                .with_filter(
                    filter::Targets::new().with_target("not_sure", filter::LevelFilter::DEBUG),
                ),
        )
        .with(
            fmt::layer()
                .json()
                .with_writer(std::sync::Arc::new(log_analysis_file))
                .with_filter(
                    filter::Targets::new().with_target("not_sure", filter::LevelFilter::DEBUG),
                ),
        )
        .init()
}
