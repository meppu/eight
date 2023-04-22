use futures::{stream, StreamExt};
use std::{
    mem,
    path::{Path, PathBuf},
};
use tokio::fs;

mod utils;

const MAXIMUM_PARALLEL_SEARCH: usize = 1024;

pub(crate) fn create_path(path: &Path, key: &str) -> crate::Result<PathBuf> {
    if key.len() < 2 {
        return Err(crate::Error::KeyTooShort);
    } else if !utils::validate_key(key) {
        return Err(crate::Error::KeyWrongFormat);
    }

    let mut new_path = path.to_path_buf();

    for list in key.chars().collect::<Vec<char>>().chunks(2) {
        new_path.push(list.iter().collect::<String>());
    }

    new_path.push("$");

    Ok(new_path)
}

pub(crate) async fn write(path: &mut PathBuf, content: String) -> crate::Result<()> {
    let file = path.file_name().unwrap().to_str().unwrap().to_string();

    path.pop();

    if !exists(path).await? && fs::create_dir_all(&path).await.is_err() {
        return Err(crate::Error::CreateDirFail);
    }

    path.push(file);

    if fs::write(&path, content).await.is_err() {
        Err(crate::Error::FileWriteFail)
    } else {
        Ok(())
    }
}

pub(crate) async fn read(path: &PathBuf) -> crate::Result<String> {
    if let Ok(value) = fs::read_to_string(path).await {
        Ok(value)
    } else {
        Err(crate::Error::FileReadFail)
    }
}

pub(crate) async fn delete(path: &PathBuf) -> crate::Result<()> {
    if let Ok(value) = fs::remove_file(path).await {
        Ok(value)
    } else {
        Err(crate::Error::FileRemoveFail)
    }
}

pub(crate) async fn exists(path: &PathBuf) -> crate::Result<bool> {
    if let Ok(value) = fs::try_exists(&path).await {
        Ok(value)
    } else {
        Err(crate::Error::CheckExistsFail)
    }
}

pub(crate) async fn flush(path: &PathBuf) -> crate::Result<()> {
    if let Ok(value) = fs::remove_dir_all(path).await {
        Ok(value)
    } else {
        Err(crate::Error::DirRemoveFail)
    }
}

pub(crate) async fn search(root: &Path, key: &str) -> crate::Result<Vec<String>> {
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
        .map(|path| tokio::spawn(async move { search_recursive(path, deep) }))
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

fn search_recursive(path: PathBuf, deep: usize) -> Vec<String> {
    let Ok(paths) = path.read_dir() else {
        return Vec::new();
    };

    let mut collected = Vec::new();

    for entry in paths.flatten() {
        let mut path = entry.path();

        if path.is_dir() {
            collected.extend(search_recursive(path, deep + 1));
        } else if path.is_file() {
            path.pop();

            let mut iter = path.iter();
            let file = (0..deep + 1)
                .map(|_| iter.next_back().unwrap().to_str().unwrap().to_string())
                .collect::<Vec<_>>()
                .iter_mut()
                .rev()
                .map(mem::take)
                .collect::<Vec<_>>()
                .join("");

            collected.push(file);
        }
    }

    collected
}
