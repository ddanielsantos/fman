use std::fs::File;

use color_eyre::{eyre::Context, Result};
use tracing::Level;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard};
use tracing_subscriber::EnvFilter;

pub fn initialize_logging() -> Result<WorkerGuard> {
    let file = File::create("tracing.log").wrap_err("failed to create tracing.log")?;
    let (non_blocking, guard) = non_blocking(file);

    // By default, the subscriber is configured to log all events with a level of `DEBUG` or higher,
    // but this can be changed by setting the `RUST_LOG` environment variable.
    let env_filter = EnvFilter::builder()
        .with_default_directive(Level::DEBUG.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(env_filter)
        .init();
    Ok(guard)
}
