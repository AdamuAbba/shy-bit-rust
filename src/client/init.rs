use bitcoincore_rpc::{Auth, Client};

pub fn init_client(path: Option<&str>) -> Client {
    let username: String = String::from("shytypes");
    let password: String = String::from("ranger");

    let host = path.unwrap_or("http://localhost:2000");

    Client::new(host, Auth::UserPass(username, password)).unwrap()
}

