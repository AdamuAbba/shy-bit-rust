pub mod client;
pub mod wallet;

use clap::Parser;
use wallet::{create_wallet, get_wallet_data};

#[derive(Parser, Debug)]
#[command(version,author="Abba Adamu", about="A mini rust app to illustrate HD wallets", long_about = None)]
struct Args {
    /// wallet commands
    #[arg(short, long, default_value_t = true)]
    wallet: bool,

    /// create new wallet
    #[arg(short, long, default_value_t = true)]
    create: bool,

    /// name of new wallet
    #[arg(short, long)]
    name: String,

    /// fetch wallet information
    #[arg(short, long)]
    get_info: Option<String>,
}

fn main() {
    let args = Args::parse();

    let Args {
        wallet,
        create,
        name,
        get_info,
    } = args;

    if wallet && create && !name.is_empty() {
        create_wallet(name)
    }
}
