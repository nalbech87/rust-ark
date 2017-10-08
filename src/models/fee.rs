#![allow(non_snake_case)]

#[derive(Deserialize, Debug)]
pub struct FeeResponse {
    pub success: bool,
    pub fees: Option<Fee>
}

/// An Ark Fee Struct
#[derive(Deserialize, Debug)]
pub struct Fee {
    pub send: u64,
    pub vote: u64,
    pub secondsignature: u64,
    pub delegate: u64,
    pub multisignature: u64,
}
   
