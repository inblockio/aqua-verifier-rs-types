// pub mod models;

pub mod models {
    pub mod base64;
    pub mod chain;
    pub mod hash;
    pub mod protocol_logs;
    pub mod revision;
    pub mod stack_str;
    pub mod timestamp;
    pub mod tx_hash;
}

pub mod crypt {
    pub type Hasher512 = sha3::Sha3_512;
    pub type Hash512 = sha3::digest::Output<Hasher512>;

    pub type Hasher256 = sha3::Sha3_256;
    pub type Hash256 = sha3::digest::Output<Hasher256>;

    pub use sha3::*;
}
