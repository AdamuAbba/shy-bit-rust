use clap::{command, Arg, ArgMatches};

fn main() {
    let match_results: ArgMatches = command!()
        .arg(Arg::new("wallet"))
        .arg(Arg::new("address"))
        .get_matches();
}
