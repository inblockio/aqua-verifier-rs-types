//! # types
//! This module contains all the types used in the Aqua SDK
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// The data required for the credentials of the user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialsData {
    pub mnemonic: String,
    pub nostr_sk: String,
    pub did_key: String,
    pub alchemy_key: String,
    pub witness_eth_network: String,
    pub witness_method: String,
}

/// The data required for the Aqua operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquaOperationData {
    pub aqua_trees: Vec<AquaTree>,
    pub aqua_tree: Option<AquaTree>,
    pub log_data: Vec<LogData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileVerificationGraphData {
    pub is_validation_successful: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormKeyGraphData {
    pub form_key: String,
    pub content: String,
    pub is_validation_successful: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormVerificationGraphData {
    pub form_keys: Vec<FormKeyGraphData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureVerificationGraphData {
    pub wallet_address: String,
    pub chain_hash_is_valid: bool,
    pub signature: String,
    pub signature_type: String,
    pub is_validation_successful: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessVerificationGraphData {
    pub tx_hash: String,
    pub merkle_root: String,
    pub is_validation_successful: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkVerificationGraphData {
    pub is_validation_successful: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "revision_type")]
pub enum RevisionGraphInfo {
    #[serde(rename = "file")]
    File(FileVerificationGraphData),
    #[serde(rename = "witness")]
    Witness(WitnessVerificationGraphData),
    #[serde(rename = "signature")]
    Signature(SignatureVerificationGraphData),
    #[serde(rename = "form")]
    Form(FormVerificationGraphData),
    #[serde(rename = "link")]
    Link(LinkVerificationGraphData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationGraphData {
    pub hash: String,
    pub previous_verification_hash: String,
    pub timestamp: String,
    pub is_validation_successful: bool,
    pub revision_type: RevisionType,
    pub info: RevisionGraphInfo,
    pub verification_graph_data: Vec<VerificationGraphData>,
    pub link_verification_graph_data: Vec<VerificationGraphData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RevisionType {
    File,
    Witness,
    Signature,
    Form,
    Link,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WitnessType {
    Tsa,
    Eth,
    Nostr,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WitnessPlatformType {
    Cli,
    Metamask,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WitnessNetwork {
    Sepolia,
    Mainnet,
    Holesky,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SignType {
    Cli,
    Metamask,
    Did,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WitnessEnvironment {
    Node,
    Browser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormVerificationResponseData {
    pub is_ok: bool,
    pub logs: Vec<LogData>,
    pub form_keys_graph_data: Vec<FormKeyGraphData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileObject {
    pub file_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_content_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_content_tree: Option<AquaTree>,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum LogType {
    Success,
    Info,
    Error,
    FinalError,
    Warning,
    Hint,
    DebugData,
    Arrow,
    File,
    Link,
    Signature,
    Witness,
    Form,
    Scalar,
    Empty,
    Tree,
}

// Using once_cell::sync::Lazy instead of lazy_static
pub static LOG_TYPE_EMOJIS: Lazy<HashMap<LogType, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(LogType::Success, "✅");
    m.insert(LogType::Info, "✨");
    m.insert(LogType::Error, "❌");
    m.insert(LogType::FinalError, "❌");
    m.insert(LogType::Warning, "🚨");
    m.insert(LogType::Hint, "💡");
    m.insert(LogType::DebugData, "🐞");
    m.insert(LogType::Arrow, "➡️");
    m.insert(LogType::File, "📄");
    m.insert(LogType::Link, "🔗");
    m.insert(LogType::Signature, "🔏");
    m.insert(LogType::Witness, "👀");
    m.insert(LogType::Form, "📝");
    m.insert(LogType::Scalar, "⏺️ ");
    m.insert(LogType::Tree, "🌿");
    m.insert(LogType::Empty, "");
    m
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogData {
    pub log_type: LogType,
    pub log: String,
    pub ident: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionTree {
    pub hash: String,
    pub children: Vec<RevisionTree>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Revision {
    pub previous_verification_hash: String,
    pub local_timestamp: String,
    pub revision_type: RevisionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_merkle_root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_timestamp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_smart_contract_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_transaction_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_sender_account_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_merkle_proof: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<SignatureData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaves: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

pub type Revisions = HashMap<String, Revision>;
pub type FileIndex = HashMap<String, String>;
pub type FormData = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeMapping {
    pub paths: HashMap<String, Vec<String>>,
    pub latest_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquaTreeWrapper {
    pub aqua_tree: AquaTree,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_object: Option<FileObject>,
    pub revision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquaTree {
    pub revisions: Revisions,
    pub file_index: FileIndex,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree: Option<RevisionTree>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_mapping: Option<TreeMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignaturePayload {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureResult {
    pub jws: SignatureData,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureData {
    pub payload: String,
    pub signatures: Vec<SignatureItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureItem {
    pub protected: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IWitnessConfig {
    pub witness_network: String,
    pub smart_contract_address: String,
    pub witness_event_verification_hash: String,
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnObject {
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessMerkleProof {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_leaf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_leaf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessResult {
    pub witness_merkle_root: String,
    pub witness_timestamp: u64,
    pub witness_network: String,
    pub witness_smart_contract_address: String,
    pub witness_transaction_hash: String,
    pub witness_sender_account_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_merkle_proof_strings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_merkle_proof_objects: Option<Vec<WitnessMerkleProof>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasEstimateResult {
    pub error: Option<String>,
    pub has_enough_balance: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_estimate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessConfig {
    pub witness_event_verification_hash: String,
    pub witness_network: WitnessNetwork,
    pub smart_contract_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessTransactionData {
    pub transaction_hash: String,
    pub wallet_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessTSAResponse {
    pub base64_response: String,
    pub provider: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessEthResponse {
    pub transaction_hash: String,
    pub wallet_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessNostrResponse {
    pub nevent: String,
    pub npub: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessNostrVerifyResult {
    pub type_name: String,
    pub data: WitnessNostrVerifyData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessNostrVerifyData {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relays: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}