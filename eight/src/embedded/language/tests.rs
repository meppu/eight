use std::collections::HashMap;

use crate::embedded::{
    language::{lexer::Lexer, parser::CallType, parser::Parser, token::Token},
    messaging::Request,
};

#[test]
fn test_execute_lexer() {
    let mut lex = Lexer::new(
        "this is lexer # comment; \n aftercomment ; \"stringwithoutspace\"; \"string \\\"with space\";".to_string()
    );
    lex.execute();

    assert_eq!(
        lex.collect(),
        vec![
            vec![
                Token {
                    value: "this".to_string(),
                    line: 1,
                    column: 4,
                },
                Token {
                    value: "is".to_string(),
                    line: 1,
                    column: 7,
                },
                Token {
                    value: "lexer".to_string(),
                    line: 1,
                    column: 13,
                },
                Token {
                    value: "aftercomment".to_string(),
                    line: 2,
                    column: 13,
                },
            ],
            vec![Token {
                value: "stringwithoutspace".to_string(),
                line: 2,
                column: 35,
            }],
            vec![Token {
                value: "string \"with space".to_string(),
                line: 2,
                column: 58,
            },]
        ]
    );
}

#[test]
fn test_execute_parser() {
    let mut env = HashMap::new();

    let a = "A".to_string();
    let b = "B".to_string();
    let c = 1;

    env.insert("varA".to_string(), a.clone());
    env.insert("varB".to_string(), b.clone());
    env.insert("varC".to_string(), c.to_string());
    let mut parser = Parser::new(env);

    assert_eq!(
        parser.execute(tokenize("set $varA $varB")).unwrap(),
        CallType::Await(Request::Set(a.clone(), b.clone()))
    );

    assert_eq!(
        parser.execute(tokenize("get $varA")).unwrap(),
        CallType::Await(Request::Get(a.clone()))
    );

    assert_eq!(
        parser.execute(tokenize("delete $varA")).unwrap(),
        CallType::Await(Request::Delete(a.clone()))
    );

    assert_eq!(
        parser.execute(tokenize("incr $varA $varC")).unwrap(),
        CallType::Await(Request::Increment(a.clone(), c))
    );

    assert_eq!(
        parser.execute(tokenize("decr $varA $varC")).unwrap(),
        CallType::Await(Request::Decrement(a.clone(), c))
    );

    assert_eq!(
        parser.execute(tokenize("search $varA")).unwrap(),
        CallType::Await(Request::Search(a.clone()))
    );

    assert_eq!(
        parser.execute(tokenize("flush")).unwrap(),
        CallType::Await(Request::Flush)
    );

    assert_eq!(
        parser.execute(tokenize("downgrade")).unwrap(),
        CallType::Await(Request::DowngradePermission)
    );

    assert_eq!(
        parser.execute(tokenize("set? $varA $varB")).unwrap(),
        CallType::Spawn(Request::Set(a.clone(), b.clone()))
    );
}

// starting here, internal functions for testing only
fn tokenize(input: &str) -> Vec<Token> {
    input
        .split(' ')
        .map(|t| Token {
            value: t.to_string(),
            line: 1,
            column: 1,
        })
        .collect()
}
