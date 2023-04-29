use clap::{value_parser, Parser};
use std::net::Ipv4Addr;

/// Simple program host eight on your server.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Path to create storage.
    /// Will create memory storage if none.
    #[arg(short, long)]
    pub directory: Option<String>,

    /// Permission level as number.
    /// Guest (0), Admin (1), Owner (2)
    #[arg(long, default_value_t = 2)]
    pub permission: u8,

    /// Port to expose.
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    /// IPv4 adress to listen.
    #[arg(short, long, default_value_t = Ipv4Addr::new(0, 0, 0, 0), value_parser = value_parser!(Ipv4Addr))]
    pub bind: Ipv4Addr,
}
