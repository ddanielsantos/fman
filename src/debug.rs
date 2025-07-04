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
use tracing_subscriber::fmt::time::OffsetTime;

lazy_static! {
    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
    pub static ref DATA_FOLDER: Option<PathBuf> =
        std::env::var(format!("{}_DATA", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
    pub static ref LOG_ENV: String = format!("{}_LOGLEVEL", PROJECT_NAME.clone());
    pub static ref LOG_FILE: String = format!("{}.log", env!("CARGO_PKG_NAME"));
}

pub fn get_dir_and_log_file_path() -> (PathBuf, PathBuf) {
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
