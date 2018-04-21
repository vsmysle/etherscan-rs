use serde_json;
use reqwest::Client;
use models::account::Account;

pub struct Etherscan {
    base_url: String,
    account: Option<Account>,
    access_token: String,
}

impl Etherscan {
    fn new() -> Etherscan {
        Etherscan {
            base_url: "https://api.etherscan.io/api?".to_owned(),
            account: None,
            access_token: "YourAccessToken".to_owned(),
        }
    }
    
    fn account(&mut self, account: Account){
        self.account = Some(account)
    }

    fn access_token(&mut self, access_token: &str){
        self.access_token = access_token.to_owned();
    }

}
