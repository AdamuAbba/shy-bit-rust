use crate::client::init::init_client;
use bitcoincore_rpc::RpcApi;

pub fn create_wallet(wallet_name: String) {
    let client = init_client(None);
    let disabled_private_keys = Some(false);
    let blank = Some(false);
    let passphrase = Some("some_unsecure-passphrase");
    let avoid_reuse = Some(true);

    let new_bitcoin_wallet = client.create_wallet(
        &wallet_name,
        disabled_private_keys,
        blank,
        passphrase,
        avoid_reuse,
    );

    match new_bitcoin_wallet {
        Ok(wallet_data) => {
            println!("{:#?}", wallet_data);
        }
        Err(e) => println!("Error encountered!: {:#?}", e),
    }
}

pub fn get_wallet_data(wallet_name: String) {
    let new_host = "http://localhost:2000/wallet/".to_owned() + &wallet_name;
    let client = init_client(Some(new_host.as_str()));
    let wallet_data = client.get_wallet_info();
    match wallet_data {
        Ok(wallet_data) => {
            println!("{:#?}", wallet_data);
        }
        Err(e) => println!("Error encountered!: {:#?}", e),
    }
}
