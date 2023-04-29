use clap::Parser;
use eight::{
    embedded::{FileStorage, MemoryStorage, Permission, Server},
    expose::{self, ConfigBuilder},
};
use std::net::SocketAddr;
use tokio::signal;

mod cli;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let args = cli::Args::parse();

    let server = if let Some(directory) = args.directory {
        Server::new(FileStorage::from_path(directory))
    } else {
        Server::new(MemoryStorage::new())
    };

    let addr = SocketAddr::from((args.bind.octets(), args.port));
    let permission = match args.permission {
        0 => Ok(Permission::Guest),
        1 => Ok(Permission::Admin),
        2 => Ok(Permission::Owner),
        _ => Err("Invalid permission value. For more information, try '--help'."),
    }?;

    let config = ConfigBuilder::from_server(server)
        .set_permission(permission)
        .bind(addr)
        .collect();

    tokio::spawn(expose::expose(config));
    signal::ctrl_c().await.ok();

    Ok(())
}
