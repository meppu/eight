use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to create storage
    #[arg(short, long)]
    pub directory: String,

    /// Permission level as number.
    ///
    /// Guest (0), Admin (1), Owner (2)
    #[arg(short, long, default_value_t = 2)]
    pub permission: u8,
}
