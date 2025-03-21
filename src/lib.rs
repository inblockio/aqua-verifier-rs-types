pub mod models;




#[cfg(test)]
mod tests {
    use crate::models::CredentialsData;


    #[test]
    fn it_works() {
       let res = CredentialsData{
         mnemonic: "".to_string(),
         nostr_sk: "".to_string(),
         did_key: "".to_string(),
         alchemy_key: "".to_string(),
         witness_eth_network: "".to_string(),
         witness_method: "".to_string(),
       };
       assert_eq!(res.mnemonic, "");
    }
}