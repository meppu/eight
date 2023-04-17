use super::lexer::lex;
use crate::{EightResult, Response, Server};
use std::{collections::HashMap, str::FromStr};

#[test]

fn test_lexer() {
    let input = "set \nlmao \"hello world\"; delete $qwe;".to_string();
    let result = lex(input);

    assert_eq!(result[0].len(), 3);
    assert_eq!(result[1].len(), 2);
}

#[tokio::test]
async fn test_query() -> EightResult<()> {
    let server = Server::from_str("./query_test").unwrap();
    server.start().await;

    let mut env = HashMap::<String, String>::new();
    env.insert("user".into(), "icecat".into());
    env.insert("val".into(), "hello world!".into());

    let results = server
        .query(
            r#"
        # an example comment
        set $user $val; # hello!
        get $user;
        # delete $user (commented out)
        flush;
        "#,
            env,
        )
        .await?;

    assert_eq!(results[0], Response::Ok);
    assert_eq!(results[1], Response::Value("hello world!".to_string()));

    Ok(())
}
