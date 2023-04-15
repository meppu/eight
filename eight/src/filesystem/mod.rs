mod utils;

use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

pub fn create_path(path: &PathBuf, key: &str) -> Result<PathBuf> {
    if key.len() < 2 {
        return Err(anyhow::anyhow!("Key must be at least 2 characters long"));
    } else if !utils::validate_key(&key) {
        return Err(anyhow::anyhow!("Key must be an alphanumeric value"));
    }

    let mut new_path = path.clone();
    for list in key.chars().collect::<Vec<char>>().chunks(2) {
        new_path.push(list.iter().collect::<String>());
    }

    Ok(new_path)
}

pub async fn write(path: &mut PathBuf, content: String) -> Result<()> {
    let file = path.file_name().unwrap().to_str().unwrap().to_string();

    if !fs::try_exists(&path).await? {
        path.pop();
        fs::create_dir_all(&path).await?;
        path.push(file);
    }

    fs::write(&path, content).await?;
    Ok(())
}

pub async fn read(path: &PathBuf) -> Result<String> {
    Ok(fs::read_to_string(path).await?)
}

pub async fn delete(path: &PathBuf) -> Result<()> {
    Ok(fs::remove_file(path).await?)
}

pub async fn flush(path: &PathBuf) -> Result<()> {
    Ok(fs::remove_dir_all(path).await?)
}
