#![allow(non_snake_case)]

#[derive(Deserialize, Debug)]
pub struct PeerResponse {
    pub success: bool,
    pub peer: Option<Peer>
}

#[derive(Deserialize, Debug)]
pub struct PeersResponse {
    pub success: bool,
    pub peers: Option<Vec<Peer>>
}

#[derive(Deserialize, Debug)]
pub struct Peer {
    pub ip: String,
    pub port: u64,
    pub version: String,
    pub errors : u64,
    pub os: String,
    pub height: Option<u64>,
    pub rate : Option<u64>,
    pub status : String,
    pub delay: u64,
}
