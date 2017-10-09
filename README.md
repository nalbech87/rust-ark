<h1 align="center">Rust Ark</h1>

<p align="center">
    <img src="https://img.shields.io/crates/v/ark.svg"/>
</p>

## Overview

`Rust-Ark` is a wrapper for the Ark Ecosystem API written in Rust. An example application can be found [here](https://github.com/nalbech87/rust-ark-example).


## License

`Rust-Ark` is under MIT license. See the LICENSE file for more info.

## Installation

In your `Cargo.toml` file, add the following to your dependancy:

```toml
[dependencies]
ark = "0.1.1"
```
## Usage

Before using `Rust-Ark`, add the following to your declarations:

```rust
extern crate ark;

use ark::manager;
use ark::models;
```

To access the Ark API's, initialize a manager:

```rust
let manager = manager::Manager::default();
```
With the manager, you can access any of the API's

### Account

Access an account:

```rust
let result = manager.get_account("AZHXnQAYajd3XkxwwiL6jnLjtDHjtAATtR");

match result {
Ok(response) => println!("Account: {:?}", response),
Err(e) => println!("Error: {:?}", e),

// AccountResponse { success: true, 
// account: Some(Account { address: "AZHXnQAYajd3XkxwwiL6jnLjtDHjtAATtR",
// publicKey: "036edc75a117164a9b46dd284a3e667bc8090ca382bdcee3bfd7f12b16eb4b2b2b", 
// unconfirmedSignature: Some(0),
// secondSignature: Some(0),
// multisignatures: Some([]), 
// secondPublicKey: None, u_multisignatures: 
// Some([]), 
// balance: "1713771029", 
// unconfirmedBalance: "1713771029" }) }
```
### Delegate

Fetch a delegate:

```rust
let result = manager.get_delegate("jarunik");

match result {
Ok(response) => println!("Account: {:?}", response),
Err(e) => println!("Error: {:?}", e),

// DelegateResponse { success: true, delegate: 
// Some(Delegate { username: "jarunik",
// address: "Aasu14aTs9ipZdy1FMv7ay1Vqn3jPskA8t", 
// publicKey: "02c7455bebeadde04728441e0f57f82f972155c088252bf7c1365eb0dc84fbf5de",
// vote: "136382464736623", 
// producedblocks: 32647, 
// missedblocks: 68, 
// rate: 21, 
// approval: 1.05, 
// productivity: 99.79 }) }
```
Get a list of forging Delegates:

```rust
let result = manager.get_delegates();

match result {
Ok(response) => println!("Account: {:?}", response),
Err(e) => println!("Error: {:?}", e),

// Return list of delegates
```

### Transaction

Fetch a transaction:

```rust
let result = manager.get_transaction("57f12f973b2f5a5c2d6f8e8eb9e189bb4fb1f694155de801b34a9f0b5d31bba6");

match result {
Ok(response) => println!("Account: {:?}", response),
Err(e) => println!("Error: {:?}", e),

// TransactionResponse { success: true, 
// transaction: Some(Transaction { 
// id: "57f12f973b2f5a5c2d6f8e8eb9e189bb4fb1f694155de801b34a9f0b5d31bba6", 
// blockid: "16808720508952111836", 
// height: Some(2159971), 
// timestamp: 17408271, 
// amount: 110282642538, 
// fee: 10000000, 
// senderId: "AGN1CKyRq6McXVAbFt75p1kowbzAQUSwYZ", 
// recipientId: "Aee8YPgbiG4gQikzDpncZb13PyPY5kZmV9", 
// senderPublicKey: "03acffa9778bdf6f6c33f297ab604ad0e28ee488a2d3c28e5e62d03e1e3168fbc4", 
// signature: "3045022100ba75293aeb1cdfcd29b9ed4455208c15237ce1b39782cdb230d51ada0dd070da022008bd802f4437f92df337124f2cee588a35e542b84d9fb6ba7b919cd2a2977663", 
// confirmations: 64, 
// vendorField: None }) }
```

### Block

Fetch a block:

```rust
let result = manager.get_block("4995325027235117218");
    
match result {
Ok(response) => println!("Account: {:?}", response),
Err(e) => println!("Error: {:?}", e),

// BlockResponse { success: true, 
// block: Some(Block { 
// id: "4995325027235117218", 
// version: 0, 
// timestamp: 17408880, 
// height: 2160046, 
// previousBlock: "4370052495680305661", 
// numberOfTransactions: 0, 
// totalAmount: 0, 
// totalFee: 0, 
// reward: 200000000, 
// payloadLength: 0, 
// payloadHash: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
// generatorPublicKey: "024ee07e2ab5c699342f0a8f5e5554439f3c5ab8ace6648fdc74b06502ed4cdf16", 
// generatorId: "AKw1H9p8yxDCwTup9fj6jUfAW9u6toTE1p", 
// blockSignature: "3045022100c447639368903b7ff1ec991d9da60b8599d08a2bf49119c0f487fc9e24465e3502204754e059280b2395e50a60266499243faff2562e3230972d09948b9ec72d3804", 
// confirmations: 7, 
//totalForged: "200000000" }) }
```

### Fees

```rust
let result = manager.get_fees();

match result {
Ok(response) => println!("Account: {:?}", response),
Err(e) => println!("Error: {:?}", e),

// FeeResponse { success: true, 
// fees: Some(Fee { 
// send: 10000000, 
// vote: 100000000, 
// secondsignature: 500000000, 
// delegate: 2500000000, 
// multisignature: 500000000 }) }
```