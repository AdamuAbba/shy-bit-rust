// use bitcoincore_rpc::{Auth, Client, RpcApi};
use clap::Parser;

/// A mini rust app to illustrate HD wallets
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Generate new wallet
    #[arg(short, long, default_value_t = true)]
    wallet: bool,

    /// create new address
    #[arg(short, long, default_value_t = true)]
    address: bool,
}

fn main() {
    let args: Args = Args::parse();

    if args.wallet {
        println!("creating new wallet")
    }
}
