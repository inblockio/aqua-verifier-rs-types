// use crate::models::hash::Hash;
use crate::models::revision::Revision;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AquaChain {
    #[serde(with = "tuple_vec_map")]
    pub file_index: Vec<(String, String)>,
    // #[serde_as(as = "Vec<(_, _)>")]
    // #[serde(with = "tuple_list")]
    #[serde(with = "tuple_vec_map")]
    pub revisions: Vec<(String, Revision)>,
}
