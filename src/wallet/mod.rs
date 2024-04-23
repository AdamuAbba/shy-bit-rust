    use crate::client::init::init_client;
    
    use bitcoincore_rpc::Client;
    use bitcoincore_rpc::RpcApi;



    pub fn create_wallet(authorize_rpc_client: Client) {
        let wallet_name = String::from("my_first_btc_wallet");
        let disabled_private_keys = Some(false);
        let blank = Some(false);
        // we are not using a passphrase so it's blank
        let passphrase = Some("");
        let avoid_reuse = Some(true);
    
        let new_bitcoin_wallet = authorize_rpc_client.create_wallet(
            &wallet_name,
            disabled_private_keys,
            blank,
            passphrase,
            avoid_reuse,
        );

        match new_bitcoin_wallet {
            Ok(wallet_data) => {
                //println!("Wallet created {:?}", wallet_data);
                let new_host = "http://localhost:2000/wallet/".to_owned()+&wallet_data.name;
                get_wallet_data(init_client(Some(new_host.as_str())));
            }
            Err(e) => println!("Error encountered!: {}", e),
        }
    }


    pub fn get_wallet_data(authorize_rpc_client: Client) {
        let wallet_data = authorize_rpc_client.get_wallet_info();
        match wallet_data {
            Ok(wallet_data) => {
                println!("{:?}",wallet_data);
            }
            Err(e) => println!("Error encountered!: {}", e),
        }
    }