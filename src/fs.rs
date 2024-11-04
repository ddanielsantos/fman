use color_eyre::{eyre::Context, Result};
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

pub fn change_dir<CB>(new_path: &Path, mut cb: CB) -> Result<()>
where
    CB: FnMut(),
{
    let res = std::env::set_current_dir(new_path).wrap_err("Failed to change directory");
    cb();

    res
}

pub fn current_dir() -> Result<PathBuf> {
    std::env::current_dir().wrap_err("Failed to get the current dir")
}

pub fn dir_entry_to_string(de: &DirEntry) -> String {
    de.path().display().to_string()
}

pub fn get_content<P: AsRef<Path>>(path: P, show_hidden: bool) -> Vec<DirEntry> {
    fs::read_dir(path)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|de| {
                    if show_hidden {
                        true
                    } else {
                        is_not_hidden(&de.path())
                    }
                })
                .collect()
        })
        .unwrap_or_else(|_| Vec::new())
}

pub fn is_not_hidden(path: &Path) -> bool {
    !is_hidden(path)
}

pub fn is_hidden(path: &Path) -> bool {
    #[cfg(target_family = "unix")]
    {
        use std::ffi::OsStr;
        if path
            .file_name()
            .and_then(OsStr::to_str)
            .is_some_and(|s| s.starts_with('.'))
        {
            return true;
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::os::macos::fs::MetadataExt;

        const UF_HIDDEN: u32 = 0x8000;
        let metadata = std::fs::metadata(path)
            .wrap_err("Failed to get metadata from path")
            .unwrap();

        if (metadata.st_flags() & UF_HIDDEN) == UF_HIDDEN {
            return true;
        }
    }

    #[cfg(target_family = "windows")]
    {
        use std::os::windows::fs::MetadataExt;

        const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

        let metadata = std::fs::metadata(path)
            .wrap_err("Failed to get metadata from path")
            .unwrap();

        if (metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN) == FILE_ATTRIBUTE_HIDDEN {
            return true;
        }
    }

    false
}

pub fn delete_all(items_to_delete: Vec<PathBuf>) {
    for qi in items_to_delete.iter() {
        if qi.is_file() {
            let res = std::fs::remove_file(qi);

            if res.is_err() {
                tracing::error!("{:?}", res);
            }
            continue;
        }

        if qi.is_dir() {
            let res = std::fs::remove_dir_all(qi);

            if res.is_err() {
                tracing::error!("{:?}", res);
            }
            continue;
        }
    }
}

pub fn create_path<P: Into<PathBuf>>(path: P) {
    let path: &PathBuf = &path.into();

    if path
        .to_string_lossy()
        .ends_with(std::path::MAIN_SEPARATOR_STR)
    {
        if let Err(e) = std::fs::create_dir_all(path) {
            tracing::error!("Could not create dir {}", e);
            return;
        }
    }

    let parent = path.parent().unwrap();
    if let Err(e) = std::fs::create_dir_all(parent) {
        tracing::error!("Could not create dir {}", e);
        return;
    }

    if let Err(e) = std::fs::File::create(path) {
        tracing::error!("Could not create file {}", e);
    };
}
