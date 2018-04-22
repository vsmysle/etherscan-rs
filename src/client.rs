use serde_json;
use std::error::Error;
use reqwest;
use reqwest::{Response, StatusCode};

static ETHERSCAN_BASE_API_URL: &'static str = "https://api.etherscan.io/api?";

pub struct Client {
    access_token: String,
}

impl Client {
    pub fn new(access_token: Option<String>) -> Self {
        Client {
            access_token: access_token.unwrap_or_else(|| "".into()),
        }
    }
    
    fn get(&self, endpoint: &str) -> Result<String, Box<Error>> {
        let mut url: String = format!("{}{}", ETHERSCAN_BASE_API_URL, endpoint);
        let resp = reqwest::get(url.as_str())?;
        self.resolver(resp)
    }

    fn resolver(&self, mut resp: Response) -> Result<String, Box<Error>> {
        match resp.status() {
            StatusCode::Ok => {
                let mut resp_str = String::new();
                resp.read_to_string(&mut resp_str).unwrap();
                Ok(resp_str)
            }
        }
    }
}
