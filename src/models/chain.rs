
use serde::{Deserialize, Serialize};
use crate::models::hash::Hash;
use crate::models::revision::Revision;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AquaChain {

    pub file_index: Vec<(Hash, String)>,
    // #[serde_as(as = "Vec<(_, _)>")]
    // #[serde(with = "tuple_list")]
    #[serde(with = "tuple_vec_map")]
    pub revisions: Vec<(Hash, Revision)>,
}
