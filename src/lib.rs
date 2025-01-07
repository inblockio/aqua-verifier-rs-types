// pub mod models;

pub mod models {
    pub mod hash;
    pub mod base64;
    pub mod stack_str;
    pub mod timestamp;
    pub mod tx_hash;
    pub mod chain;
    pub mod revision;
    pub mod protocol_logs;
}

pub mod crypt {
    pub type Hasher = sha3::Sha3_512;
    pub type Hash = sha3::digest::Output<Hasher>;
    pub use sha3::*;
}

