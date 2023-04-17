use super::lexer::lex;
use crate::{language::runtime::QueryExecutor, Response, Server, Storage};
use std::{collections::HashMap, str::FromStr};

#[test]

fn test_lexer() {
    let input = "set \nlmao \"hello world\"; delete $qwe;".to_string();
    let result = lex(input);

    assert_eq!(result[0].len(), 3);
    assert_eq!(result[1].len(), 2);
}

#[tokio::test]
async fn test_runtime() -> anyhow::Result<()> {
    let storage = Storage::from_str("./runtime_test")?;
    let server = Server::new(storage);

    server.start().await;

    let input = r#"set a_key "$hello"; get a_key; flush;"#.to_string();

    let mut env = HashMap::<String, String>::new();
    env.insert("hello".into(), "wow".into());

    let mut runtime = QueryExecutor::new(input, env);
    let result = runtime.execute(&server).await?;

    assert_eq!(result, Response::Ok);

    Ok(())
}
