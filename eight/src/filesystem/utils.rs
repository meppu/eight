pub(super) fn validate_key(key: &str) -> bool {
    for character in key.chars() {
        if !character.is_alphanumeric() && character != '_' {
            return false;
        }
    }

    true
}
