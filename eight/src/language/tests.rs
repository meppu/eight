use super::lexer::Lexer;

#[test]

fn simple_lexer() {
    let simple = "set \nlmao \"hello world\"; delete $qwe;".to_string();

    let mut lexer = Lexer::new(simple);
    lexer.execute();

    let result = lexer.collect();

    assert_eq!(result[0].len(), 3);
    assert_eq!(result[1].len(), 2);
}
