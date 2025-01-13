// use crate::{crypt::Hash256, models::hash::Hash};
use crate::models::timestamp::Timestamp;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Revision {
    pub previous_verification_hash: String,
    pub nonce: String,
    pub domain_id: String,
    pub local_timestamp: Timestamp,
    pub revision_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_require_indepth_verification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_verification_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
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
    pub witness_transaction_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_sender_account_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_merkle_proof: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaves: Option<Vec<String>>,
}
