use std::path::{Path, PathBuf};
use tokio::fs;

mod utils;

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

    Ok(new_path)
}

pub(crate) async fn exists(path: &PathBuf) -> crate::Result<bool> {
    if let Ok(value) = fs::try_exists(&path).await {
        Ok(value)
    } else {
        Err(crate::Error::CheckExistsFail)
    }
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

pub(crate) async fn flush(path: &PathBuf) -> crate::Result<()> {
    if let Ok(value) = fs::remove_dir_all(path).await {
        Ok(value)
    } else {
        Err(crate::Error::DirRemoveFail)
    }
}
