use crate::models::hash::Hash;
use crate::models::timestamp::Timestamp;
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Revision {
//     pub previous_verification_hash: String,
//     pub nonce: String,
//     pub domain_id: String,
//     pub local_timestamp: Timestamp,
//     pub revision_type: String,
//     pub file_hash: Hash,
//     pub link_type: Option<String>,
//     pub link_require_indepth_verification: Option<bool>,
//     pub link_verification_hash: Option<Hash>,
//     pub link_uri: Option<String>,
//     pub signature: Option<Hash>,
//     pub signature_public_key: Option<String>,
//     pub signature_wallet_address: Option<String>,
//     pub signature_type: Option<String>,
//     pub witness_merkle_root: Option<String>,
//     pub witness_timestamp: Option<Timestamp>,
//     pub witness_network: Option<String>,
//     pub witness_smart_contract_address: Option<String>,
//     pub witness_transaction_hash: Option<Hash>,
//     pub witness_sender_account_address: Option<String>,
//     pub witness_merkle_proof: Option<String>,
//     pub leaves: Vec<Hash>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Revision {
    pub previous_verification_hash: String,
    pub nonce: String,
    pub domain_id: String,
    pub local_timestamp: Timestamp,
    pub revision_type: String,
    pub file_hash: Hash,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_require_indepth_verification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_verification_hash: Option<Hash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Hash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_merkle_root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_timestamp: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_smart_contract_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_transaction_hash: Option<Hash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_sender_account_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_merkle_proof: Option<String>,
    pub leaves: Vec<Hash>,
}
