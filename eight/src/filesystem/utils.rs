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
    if let Ok(paths) = path.read_dir() {
        paths
            .flatten()
            .map(|entry| entry.path())
            .map(|path| {
                if path.is_dir() {
                    search_recursive(path, deep + 1)
                } else if path.is_file() {
                    vec![get_file_name(path, deep)]
                } else {
                    Vec::new()
                }
            })
            .collect::<Vec<_>>()
            .concat()
    } else {
        Vec::new()
    }
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
