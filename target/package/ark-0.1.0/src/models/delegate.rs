#![allow(non_snake_case)]

#[derive(Deserialize, Debug)]
pub struct DelegateResponse {
    pub success: bool,
    pub delegate: Option<Delegate>
}

#[derive(Deserialize, Debug)]
pub struct DelegatesResponse {
    pub success: bool,
    pub delegates: Option<Vec<Delegate>>
}

/// An Ark Delegate Struct
#[derive(Deserialize, Debug)]
pub struct Delegate {
    pub username: String,
    pub address: String,
    pub publicKey: String,
    pub vote : String,
    pub producedblocks: u64,
    pub missedblocks: u64,
    pub rate : u64,
    pub approval : f64,
    pub productivity: f64,
}
