use ethers_rs::{Address, Signature, U256};
use serde::{Deserialize, Serialize};

use crate::ptlc::{ClientPTLC, ServerPTLC};

/// The contract state machine for payment channel over `EVM` like blockchain
#[derive(Debug, Serialize, Deserialize)]
pub struct ContractSM {
    /// Contract created from account address.
    pub from: Address,
    /// Contract created to account address
    pub to: Address,
    /// The `evm smart contract` address of asset which this channel transferring
    pub asset_contract: Address,
    /// erc20 token decimals.
    pub asset_decimals: U256,
    /// Asset balance of `from` address
    pub balance_from: U256,
    /// Asset balance of `to` address
    pub balance_to: U256,
    /// Contract current version, start counting from `0`
    pub version: U256,
    /// secp256k1 signature from `from` address,
    pub from_sig: Option<Signature>,
    /// secp256k1 signature from `to` address
    pub to_sig: Option<Signature>,
    /// Handshake structure for current state
    pub handshake: Handshake,
}

/// Current contract state handshake protocol
#[derive(Debug, Serialize, Deserialize)]
pub enum Handshake {
    RSMC,
    ClientPTLC(ClientPTLC),
    ServerPTLC(ServerPTLC),
}
