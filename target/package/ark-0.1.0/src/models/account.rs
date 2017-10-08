#![allow(non_snake_case)]

#[derive(Deserialize, Debug)]
pub struct AccountResponse {
    pub success: bool,
    pub account: Option<Account>
}


/// An Ark Account Struct
#[derive(Deserialize, Debug)]
pub struct Account {
    pub address: String,
    pub publicKey: String,
    pub unconfirmedSignature: Option<u64>,
    pub secondSignature: Option<u64>,
    pub multisignatures: Option<Vec<String>>,
    pub secondPublicKey : Option<String>,
    pub u_multisignatures : Option<Vec<String>>,
    pub balance: String,
    pub unconfirmedBalance: String
}
