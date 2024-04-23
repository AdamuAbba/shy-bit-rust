pub mod client;
pub mod wallet;

use wallet::create_wallet;
use client::init::init_client;

use clap::{Parser, Subcommand};



/// A mini rust app to illustrate HD wallets
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// wallet commands
    #[arg(short, long, default_value_t = true)]
    wallet: bool,

    /// address commands
    #[arg(short, long, default_value_t = true)]
    address: bool,

    #[command(subcommand)]
    sub_commands: Option<SubCommands>,
}


#[derive(Subcommand,Debug)]
enum SubCommands {
    /// creates a new instance
    Create {
        /// name of wallet or address
        #[arg(short, long, default_value_t = String::from("my_first_btc_wallet"))]
        name:String
    }
    }
    


fn main() {
    let args: Args = Args::parse();

    if args.wallet {
       create_wallet(init_client(None))
    }
}
