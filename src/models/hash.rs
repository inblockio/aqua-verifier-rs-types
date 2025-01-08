use crate::models::stack_str::{from_hex, StackStr};
#[derive(Hash, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
/// Used to represent Hashes
pub struct Hash(crate::crypt::Hash256);

impl core::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_stackstr()[..])
        // f.write_fmt(format_args!("{}..", &self.to_stackstr()[..20]))
    }
}

impl Hash {
    pub fn to_stackstr(self) -> StackStr<128> {
        let mut arr = [0; 128];
        // Safety: data is exactly the right size for the hex output
        unsafe {
            hex::encode_to_slice(self.0, &mut arr[..]).unwrap_unchecked();
        }
        // StackStr(arr)
        StackStr::new(arr)
    }
}

impl std::str::FromStr for Hash {
    // todo: err
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hash(from_hex(s).ok_or(())?.into()))
    }
}

impl From<[u8; 64]> for Hash {
    fn from(value: [u8; 64]) -> Self {
        crate::crypt::Hash256::from(value).into()
    }
}
impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = [0u8; 64 * 2];
        // Safety: data is exactly the right size for the hex output
        unsafe {
            hex::encode_to_slice(<[u8; 64]>::from(self.0), &mut data).unwrap_unchecked();
        }
        f.write_str(StackStr::new(data).as_ref())
    }
}

impl std::ops::Deref for Hash {
    type Target = crate::crypt::Hash256;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::crypt::Hash256> for Hash {
    fn from(value: crate::crypt::Hash256) -> Self {
        Self(value)
    }
}

// impl From<[u8; 64]> for Hash {
//     fn from(value: [u8; 64]) -> Self {
//         Into::<crate::crypt::Hash>::into(value).into()
//     }
// }
impl From<Hash> for crate::crypt::Hash256 {
    fn from(val: Hash) -> Self {
        val.0
    }
}

impl<'de> serde::Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <std::borrow::Cow<'de, str>>::deserialize(deserializer)?;
        s.parse()
            .map_err(|_| serde::de::Error::custom("Invalid sha3_512 hash"))
    }
}

impl serde::Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.0[..]))
    }
}
