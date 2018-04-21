pub struct Account {
    pub address: String,
}

impl Account {
   pub fn new(address: &str) -> Account {
        Account {
            address: address.to_owned(),
        }
    }
}

