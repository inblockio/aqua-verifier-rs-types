
use serde::{Deserialize, Serialize};
use crate::models::hash::Hash;
use crate::models::timestamp::Timestamp;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Revision {
    previous_verification_hash :  String,
    nonce : String,
    domain_id : String,
    local_timestamp : Timestamp,
    revision_type : String,
    file_hash : Hash ,

    link_type: Option<String>,
    link_require_indepth_verification: Option<bool>,
    link_verification_hash: Option<Hash>,
    link_uri: Option<String>,


    signature: Option<Hash>,
    signature_public_key: Option<String>,
    signature_wallet_address: Option<String>,
    signature_type: Option<String>,


    witness_merkle_root: Option<String>,
    witness_timestamp: Option<Timestamp>,
    witness_network: Option<String>,
    witness_smart_contract_address: Option<String>,
    witness_transaction_hash: Option<Hash>,
    witness_sender_account_address: Option<String>,
    witness_merkle_proof: Option<String>,

    leaves : Vec<Hash>
}