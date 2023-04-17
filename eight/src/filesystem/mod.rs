use crate::EightError;
use std::path::PathBuf;
use tokio::fs;

mod utils;

pub(crate) fn create_path(path: &PathBuf, key: &str) -> Result<PathBuf, EightError> {
    if key.len() < 2 {
        return Err(EightError::KeyTooShort);
    } else if !utils::validate_key(&key) {
        return Err(EightError::KeyWrongFormat);
    }

    let mut new_path = path.clone();
    for list in key.chars().collect::<Vec<char>>().chunks(2) {
        new_path.push(list.iter().collect::<String>());
    }

    Ok(new_path)
}

pub(crate) async fn exists(path: &PathBuf) -> Result<bool, EightError> {
    if let Ok(value) = fs::try_exists(&path).await {
        Ok(value)
    } else {
        Err(EightError::CheckExistsFail)
    }
}

pub(crate) async fn write(path: &mut PathBuf, content: String) -> Result<(), EightError> {
    let file = path.file_name().unwrap().to_str().unwrap().to_string();

    path.pop();

    if !exists(&path).await? {
        if fs::create_dir_all(&path).await.is_err() {
            return Err(EightError::CreateDirFail);
        }
    }

    path.push(file);

    if fs::write(&path, content).await.is_err() {
        Err(EightError::FileWriteFail)
    } else {
        Ok(())
    }
}

pub(crate) async fn read(path: &PathBuf) -> Result<String, EightError> {
    if let Ok(value) = fs::read_to_string(path).await {
        Ok(value)
    } else {
        Err(EightError::FileReadFail)
    }
}

pub(crate) async fn delete(path: &PathBuf) -> Result<(), EightError> {
    if let Ok(value) = fs::remove_file(path).await {
        Ok(value)
    } else {
        Err(EightError::FileRemoveFail)
    }
}

pub(crate) async fn flush(path: &PathBuf) -> Result<(), EightError> {
    if let Ok(value) = fs::remove_dir_all(path).await {
        Ok(value)
    } else {
        Err(EightError::DirRemoveFail)
    }
}
