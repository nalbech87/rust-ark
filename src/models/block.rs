#![allow(non_snake_case)]

#[derive(Deserialize, Debug)]
pub struct BlockResponse {
    pub success: bool,
    pub block: Option<Block>
}

#[derive(Deserialize, Debug)]
pub struct BlocksResponse {
    pub success: bool,
    pub blocks: Option<Vec<Block>>
}

#[derive(Deserialize, Debug)]
pub struct BlockHeightResponse {
    pub success: bool,
    pub height: Option<u64>,
    pub id: Option<String>
}

/// An Ark Block Struct
#[derive(Deserialize, Debug)]
pub struct Block {
    pub id: String,
    pub version: u64,
    pub timestamp: u64,
    pub height : u64,
    pub previousBlock : String,
    pub numberOfTransactions : u64,
    pub totalAmount: u64,
    pub totalFee: u64,
    pub reward: u64,
    pub payloadLength: u64,
    pub payloadHash : String,
    pub generatorPublicKey : String,
    pub generatorId : String,
    pub blockSignature : String,
    pub confirmations: u64,
    pub totalForged: String
}
   
