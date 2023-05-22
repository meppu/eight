#[derive(Debug, PartialEq)]
pub(super) struct Token {
    pub(super) value: String,
    pub(super) line: usize,
    pub(super) column: usize,
}
