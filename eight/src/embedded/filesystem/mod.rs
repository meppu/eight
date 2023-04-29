use futures::{stream, StreamExt};
use std::path::{Path, PathBuf};
use tokio::fs;

mod utils;

const MAXIMUM_PARALLEL_SEARCH: usize = 512;

pub(crate) fn create_path(path: &Path, key: &str) -> super::Result<PathBuf> {
    if key.len() < 2 {
        return Err(super::Error::KeyTooShort);
    } else if !utils::validate_key(key) {
        return Err(super::Error::KeyWrongFormat);
    }

    let mut new_path = path.to_path_buf();

    for list in key.chars().collect::<Vec<char>>().chunks(2) {
        new_path.push(list.iter().collect::<String>());
    }

    new_path.push("$");

    Ok(new_path)
}

pub(crate) async fn write(path: &mut PathBuf, content: String) -> super::Result<()> {
    let file = path.file_name().unwrap().to_str().unwrap().to_string();

    path.pop();

    if !exists(path).await? && fs::create_dir_all(&path).await.is_err() {
        return Err(super::Error::CreateDirFail);
    }

    path.push(file);

    fs::write(&path, content)
        .await
        .map_err(|_| super::Error::FileWriteFail)
}

pub(crate) async fn read(path: &PathBuf) -> super::Result<String> {
    fs::read_to_string(path)
        .await
        .map_err(|_| super::Error::FileReadFail)
}

pub(crate) async fn delete(path: &PathBuf) -> super::Result<()> {
    fs::remove_file(path)
        .await
        .map_err(|_| super::Error::FileRemoveFail)
}

pub(crate) async fn exists(path: &PathBuf) -> super::Result<bool> {
    fs::try_exists(path)
        .await
        .map_err(|_| super::Error::CheckExistsFail)
}

pub(crate) async fn flush(path: &PathBuf) -> super::Result<()> {
    fs::remove_dir_all(path)
        .await
        .map_err(|_| super::Error::DirRemoveFail)
}

pub(crate) async fn search(root: &Path, key: &str) -> super::Result<Vec<String>> {
    let key_length = key.len();
    let deep = key_length / 2;

    let search_path = if key_length == 0 {
        root.to_path_buf()
    } else {
        let mut path = create_path(root, key)?;
        path.pop();

        if key_length % 2 == 1 {
            path.pop();
        }

        path
    };

    let Ok(paths) = search_path.read_dir() else {
        return Ok(Vec::new());
    };

    let tasks = stream::iter(paths)
        .filter_map(|path| async {
            if let Ok(entry) = path {
                let path = entry.path();

                if path.is_dir() {
                    return Some(entry.path());
                }
            }

            None
        })
        .map(|path| tokio::spawn(async move { utils::search_recursive(path, deep) }))
        .buffer_unordered(MAXIMUM_PARALLEL_SEARCH);

    let results = tasks
        .filter_map(|value| async {
            match value {
                Ok(result) => Some(result),
                _ => None,
            }
        })
        .collect::<Vec<_>>()
        .await
        .concat();

    Ok(results)
}
