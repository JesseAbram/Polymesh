use parity_scale_codec::{Decode, Encode};
use rstd::{
    cmp::{Ord, PartialOrd},
    convert::TryFrom,
    default::Default,
    prelude::Vec,
};

/// Size of key, when it is u64
const KEY_SIZE_TEST: usize = 8;
const KEY_SIZE: usize = 32;

/// Signing key type.
#[allow(missing_docs)]
#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, Debug)]
pub enum KeyType {
    External,
    Identity,
    Multisig,
    Relayer,
    Custom(u8),
}

impl Default for KeyType {
    fn default() -> Self {
        KeyType::External
    }
}

/// It stores a simple key.
/// It uses fixed size to avoid dynamic memory allocation.
#[derive(Encode, Decode, Default, PartialOrd, Ord, Eq, Clone, Debug)]
pub struct Key([u8; KEY_SIZE]);

impl TryFrom<Vec<u8>> for Key {
    type Error = &'static str;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Key::try_from(v.as_slice())
    }
}

impl TryFrom<&Vec<u8>> for Key {
    type Error = &'static str;

    fn try_from(v: &Vec<u8>) -> Result<Self, Self::Error> {
        Key::try_from(v.as_slice())
    }
}

impl TryFrom<&str> for Key {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Key::try_from(s.as_bytes())
    }
}

impl TryFrom<&[u8]> for Key {
    type Error = &'static str;

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        let mut k = Key::default();
        match s.len() {
            KEY_SIZE => k.0.copy_from_slice(s),
            KEY_SIZE_TEST => k.0[..KEY_SIZE_TEST].copy_from_slice(s),
            _ => return Err("Invalid size for a key"),
        };
        Ok(k)
    }
}

impl From<[u8; KEY_SIZE]> for Key {
    fn from(s: [u8; KEY_SIZE]) -> Self {
        Key(s)
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<&[u8]> for Key {
    fn eq(&self, other: &&[u8]) -> bool {
        match other.len() {
            KEY_SIZE => self.0 == *other,
            KEY_SIZE_TEST => {
                self.0[..KEY_SIZE_TEST] == **other
                    && self.0[KEY_SIZE_TEST..] == [0u8; KEY_SIZE - KEY_SIZE_TEST]
            }
            _ => false,
        }
    }
}

impl PartialEq<Vec<u8>> for Key {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self == &other.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::{Key, KEY_SIZE};
    use std::convert::TryFrom;

    #[test]
    fn build_test() {
        let k: [u8; KEY_SIZE] = [1u8; KEY_SIZE];
        let k2 = "ABCDABCD".as_bytes().to_vec();

        assert!(Key::try_from(k).is_ok());
        assert!(Key::try_from(k2.as_slice()).is_ok());
        assert!(Key::try_from(k2).is_ok());

        assert!(Key::try_from("ABCDABCDx".as_bytes()).is_err());
    }
}
