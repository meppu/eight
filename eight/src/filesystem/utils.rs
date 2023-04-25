use std::{mem, path::PathBuf};

pub(super) fn validate_key(key: &str) -> bool {
    for character in key.chars() {
        if !character.is_alphanumeric() && character != '_' {
            return false;
        }
    }

    true
}

pub(super) fn search_recursive(path: PathBuf, deep: usize) -> Vec<String> {
    let mut result = Vec::new();

    let Ok(entries) = path.read_dir() else { 
        return result; 
    };

    let paths = entries.flatten().map(|entry| entry.path());
    for path in paths {
        if path.is_dir() {
            result.extend(search_recursive(path, deep + 1))
        } else if path.is_file() {
            result.push(get_file_name(path, deep));
        }
    }

    result
}

fn get_file_name(path: PathBuf, deep: usize) -> String {
    let mut iter = path.iter();
    iter.next_back();

    (0..deep + 1)
        .map(|_| iter.next_back().unwrap().to_str().unwrap().to_string())
        .collect::<Vec<_>>()
        .iter_mut()
        .rev()
        .map(mem::take)
        .collect::<Vec<_>>()
        .join("")
}
