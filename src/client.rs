use super::models::account::Account;

pub struct Etherscan {
    pub base_url: String,
    pub account: Option<Account>,
    pub access_token: Option<String>,
}

impl Etherscan {
    fn new() -> Etherscan {
        Etherscan {
            base_url: "https://api.etherscan.io/api?",
            account: None,
            access_token: "YourAccessToken"
        }
    }
    
    fn new(account: Account) -> Etherscan {
        Etherscan {
            base_url: "https://api.etherscan.io/api?",
            account: account,
            access_token: "YourAccessToken"
        }
    }

    fn new(account: Account, access_token: String) -> Etherscan {
        Etherscan {
            base_url: "https://api.etherscan.io/api?",
            account: account,
            access_token: access_token,
    }

}
