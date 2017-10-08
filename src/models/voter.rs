 #![allow(non_snake_case)]

#[derive(Deserialize, Debug)]
pub struct VotersResponse {
    pub success: bool,
    pub accounts: Option<Vec<Voter>>
}

/// An Ark Voter Struct
#[derive(Deserialize, Debug)]
pub struct Voter {
    pub address: String,
    pub publicKey: String,
    pub username : Option<String>,
    pub balance : String
}
   
