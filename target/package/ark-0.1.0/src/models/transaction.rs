#![allow(non_snake_case)]

#[derive(Deserialize, Debug)]
pub struct TransactionResponse {
    pub success: bool,
    pub transaction: Option<Transaction>
}

#[derive(Deserialize, Debug)]
pub struct TransactionsResponse {
    pub success: bool,
    pub transactions: Option<Vec<Transaction>>
}

/// An Ark Transaction Struct
#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub id: String,
    pub blockid: String,
    pub height: Option<u64>,
    pub timestamp : u64,
    pub amount : u64,
    pub fee : u64,
    pub senderId: String,
    pub recipientId: String,
    pub senderPublicKey: String,
    pub signature: String,
    pub confirmations : u64,
    pub vendorField: Option<String>,
}
   
