use clap::Parser;
use eight::{
    embedded::{Permission, Server},
    expose::{self, ConfigBuilder},
};
use std::{net::SocketAddr, str::FromStr};

mod cli;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let args = cli::Args::parse();

    let server = Server::from_str(&args.directory).unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let permission = match args.permission {
        0 => Ok(Permission::Guest),
        1 => Ok(Permission::Admin),
        2 => Ok(Permission::Owner),
        _ => Err("Invalid permission value. For more information, try '--help'."),
    }?;

    let config = ConfigBuilder::from_server(server)
        .set_fallback("https://surrealdb.com")
        .set_permission(permission)
        .bind(addr)
        .collect();

    expose::expose(config).await;

    Ok(())
}
