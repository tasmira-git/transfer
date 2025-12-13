use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    EnvFilter, fmt::time::LocalTime, layer::SubscriberExt, util::SubscriberInitExt,
};

pub fn init_subscriber(is_debug: bool) {
    let filter = if is_debug {
        format!("info,{}={}", env!("CARGO_CRATE_NAME"), LevelFilter::DEBUG)
    } else {
        format!("info,{}={}", env!("CARGO_CRATE_NAME"), LevelFilter::INFO)
    };

    tracing_subscriber::registry()
        .with(EnvFilter::new(filter))
        .with(tracing_subscriber::fmt::layer().with_timer(LocalTime::rfc_3339()))
        .init();
}
