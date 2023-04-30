use eight::{
    embedded::{
        self,
        messaging::{Request, Response},
        server::{Permission, Server},
        storage::memory::Storage,
    },
    env,
};

#[tokio::main]
async fn main() -> embedded::Result<()> {
    let storage = Storage::new();
    let server = Server::new(storage);

    server.start().await;

    let response = server
        .call(Request::Set("user".into(), "value".into()))
        .await?;

    assert_eq!(response, Response::Ok);

    let results = server
        .query("get $key; delete $key;", env!(key: "user"))
        .await?;

    assert_eq!(results.len(), 2);
    assert_eq!(results[0], Response::Text("value".into()));
    assert_eq!(results[1], Response::Ok);

    server.set_permission(Permission::Guest).await;

    let response = server
        .call(Request::Set("user".into(), "value".into()))
        .await?;
    
    assert_eq!(
        response,
        Response::Error(embedded::Error::PermissionFailure)
    );

    Ok(())
}
