
use reqwest;
use reqwest::Error;
use models;

#[derive(Clone, Debug, Default)]
pub struct Manager {
    url : String
}

/// Implementation details
#[allow(dead_code)]
impl Manager {
 
/// Returns a new default Ark Manager stuct based on the specified identifier
    pub fn default() -> Manager {
        Manager {
            url: ("https://api.arknode.net/").to_string(),
        }
    }

/// Updates the base URL to a new Ark node
///
/// # Arguments
///
/// * `new_url` - A base url string.
///
/// # Remarks
///
/// *Note*: Format should include an ending / but not /api: 
/// Example: "https://api.arknode.net/"
    pub fn update_base_url(&mut self, new_url: &str) {
        self.url = new_url.to_string()
    }

/// Updates the base url with a constructed url from a network struct
///
/// # Arguments
///
/// * `network` - A network struct.
    pub fn update_network(&mut self, mut network: models::Network) {
        self.url = network.get_url()
    }


/// Retrieve a list of currently forging delegates
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_delegates(&self) -> Result<models::DelegatesResponse, Error> {
        let endpoint = "api/delegates";
        let final_url =  [&self.url, endpoint].join("");
        println!("{}", final_url);
        let s_slice : &str = &final_url;
        let json: models::DelegatesResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }   


/// Retrieve a particular delegate matching the name string
///
/// # Arguments
///
/// * `delegate` - The delegate string.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_delegate(&self, delegate: &str) -> Result<models::DelegateResponse, Error> {
        let endpoint = "api/delegates/get?username=";
        let final_url =  [&self.url, endpoint, delegate].join("");
        let s_slice : &str = &final_url;
        let json: models::DelegateResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }

/// Retrieve a particular account matching the Ark Address
///
/// # Arguments
///
/// * `address` - The address string.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_account(&self, address: &str) -> Result<models::AccountResponse, Error> {
        let endpoint = "api/accounts?address=";
        let final_url =  [&self.url, endpoint, address].join("");
        let s_slice : &str = &final_url;
        let json: models::AccountResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }

/// Retrieve a a list of voted delegates corresponding to an Ark address.
///
/// # Arguments
///
/// * `address` - The address string.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_votes(&self, address: &str) -> Result<models::DelegatesResponse, Error> {
        let endpoint = "api/accounts/delegates?address=";
        let final_url =  [&self.url, endpoint, address].join("");
        let s_slice : &str = &final_url;
        let json: models::DelegatesResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }

/// Retrieve a a list of accounts who voted for a particular delegate.
///
/// # Arguments
///
/// * `delegate_public_key` - The delegate's public key.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_voters(&self, delegate_public_key: &str) -> Result<models::VotersResponse, Error> {
        let endpoint = "api/delegates/voters?publicKey=";
        let final_url =  [&self.url, endpoint, delegate_public_key].join("");
        let s_slice : &str = &final_url;
        let json: models::VotersResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }


/// Retrieve a particular transaction matching the transaction id
///
/// # Arguments
///
/// * `transaction_id` - The transaction id string.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_transaction(&self, transaction_id: &str) -> Result<models::TransactionResponse, Error> {
        let endpoint = "api/transactions/get?id=";
        let final_url =  [&self.url, endpoint, transaction_id].join("");
        let s_slice : &str = &final_url;
        let json: models::TransactionResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }

/// Retrieve a a transactions a particular account received.
///
/// # Arguments
///
/// * `address` - The account's address.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_received_transactions(&self, address: &str) -> Result<models::TransactionsResponse, Error> {
        let endpoint = "api/transactions?recipientId=";
        let final_url =  [&self.url, endpoint, address].join("");
        let s_slice : &str = &final_url;
        let json: models::TransactionsResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }
/// Retrieve a a transactions a particular account sent.
///
/// # Arguments
///
/// * `address` - The account's address.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_sent_transactions(&self, address: &str) -> Result<models::TransactionsResponse, Error> {
        let endpoint = "api/transactions?senderId=";
        let final_url =  [&self.url, endpoint, address].join("");
        let s_slice : &str = &final_url;
        let json: models::TransactionsResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }

/// Retrieve a particular block matching the block id
///
/// # Arguments
///
/// * `block_id` - The block id string.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_block(&self, block_id: &str) -> Result<models::BlockResponse, Error> {
        let endpoint = "api/blocks/get?id=";
        let final_url =  [&self.url, endpoint, block_id].join("");
        let s_slice : &str = &final_url;
        let json: models::BlockResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }

/// Retrieve a list of the last blocks forged.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_blocks(&self) -> Result<models::BlocksResponse, Error> {
        let endpoint = "api/blocks";
        let final_url =  [&self.url, endpoint].join("");
        let s_slice : &str = &final_url;
        let json: models::BlocksResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }
/// Retrieve a list of blocks forged by a delegate
///
/// # Arguments
///
/// * `public_key` - The delegate's public key.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_blocks_from(&self, public_key: &str) -> Result<models::BlocksResponse, Error> {
        let endpoint = "api/blocks?generatorPublicKey=";
        let final_url =  [&self.url, endpoint, public_key].join("");
        let s_slice : &str = &final_url;
        let json: models::BlocksResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }

/// Retrieves the last blocksforged.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_last_block(&self)  -> Result<models::BlocksResponse, Error> {
        let endpoint = "api/blocks?limit=1";
        let final_url =  [&self.url, endpoint].join("");
        let s_slice : &str = &final_url;
        let json: models::BlocksResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }

/// Retrieves the current block height.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_block_height(&self)  -> Result<models::BlockHeightResponse, Error> {
        let endpoint = "api/blocks/getHeight";
        let final_url =  [&self.url, endpoint].join("");
        let s_slice : &str = &final_url;
        let json: models::BlockHeightResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }

/// Retrieves the current fees for the node you are connected to.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error

    pub fn get_fees(&self)  -> Result<models::FeeResponse, Error> {
        let endpoint = "api/blocks/getFees";
        let final_url =  [&self.url, endpoint].join("");
        let s_slice : &str = &final_url;
        let json: models::FeeResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }

/// Retrieve a list of peers on the node.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_peers(&self) -> Result<models::PeersResponse, Error> {
        let endpoint = "api/peers";
        let final_url =  [&self.url, endpoint].join("");
        let s_slice : &str = &final_url;
        let json: models::PeersResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }
/// Retrieves a peer matching the ip and port.
///
/// # Arguments
///
/// * `ip` - The peer's ip address.
/// * `port` - The peer's port.
///
/// # Remarks
///
/// Returns a Result enum containing the response or error
    pub fn get_peer(&self, ip: &str, port: &str) -> Result<models::PeerResponse, Error> {
        let endpoint = "api/peers/get?ip=";
        let final_url =  [&self.url, endpoint, ip, "&port=", port].join("");
        println!("{}", final_url);
        let s_slice : &str = &final_url;
        let json: models::PeerResponse = reqwest::get(s_slice)?.json()?;
        Ok(json)
    }
}
