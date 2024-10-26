use std::{fs::File, path::PathBuf};

use color_eyre::{eyre::Context, Result};
use directories::ProjectDirs;
use lazy_static::lazy_static;
use tracing::Level;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard};
use tracing_subscriber::{
    fmt::time::{self},
    EnvFilter,
};

lazy_static! {
    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
    pub static ref DATA_FOLDER: Option<PathBuf> =
        std::env::var(format!("{}_DATA", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
    pub static ref LOG_ENV: String = format!("{}_LOGLEVEL", PROJECT_NAME.clone());
    pub static ref LOG_FILE: String = format!("{}.log", env!("CARGO_PKG_NAME"));
}

pub fn initialize_logging() -> Result<WorkerGuard> {
    let (dir, file_path) = get_dir_and_log_file_path();
    std::fs::create_dir_all(dir)?;

    let file = File::create(file_path).wrap_err("failed to create log file")?;
    let (non_blocking, guard) = non_blocking(file);

    let env_filter = EnvFilter::builder()
        .with_default_directive(Level::DEBUG.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_file(true)
        .with_ansi(false)
        .with_timer(time::OffsetTime::local_rfc_3339().expect("could not get local time offset"))
        .with_line_number(true)
        .with_writer(non_blocking)
        .with_env_filter(env_filter)
        .init();

    Ok(guard)
}

fn get_dir_and_log_file_path() -> (PathBuf, PathBuf) {
    let dir = {
        if let Some(pt) = DATA_FOLDER.clone() {
            pt
        } else if let Some(dir) = ProjectDirs::from("com", "dd", env!("CARGO_PKG_NAME")) {
            dir.data_local_dir().to_path_buf()
        } else {
            PathBuf::from(".").join(".data")
        }
    };

    let file_path = dir.join(LOG_FILE.clone());

    (dir, file_path)
}
