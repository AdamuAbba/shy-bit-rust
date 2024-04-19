use clap::{command, Arg, ArgMatches};

fn main() {
    let match_results: ArgMatches = command!().about("This a simply a rust app to help us understand HD wallets by making use of a JSON RPC client to make calls to a bitcoin core backend.")
        .arg(
            Arg::new("wallet")
                .long("wallet")
                .help("wallet specific commands")
                .aliases(["Wallet", "wlet"]),
        )
        .arg(
            Arg::new("address")
                .long("address")
                .help("address specific commands")
                .aliases(["add", "Address"]),
        )
        .get_matches();
}
