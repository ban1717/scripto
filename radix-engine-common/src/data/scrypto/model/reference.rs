use super::OBJECT_ID_LENGTH;
use crate::data::scrypto::ScryptoCustomValueKind;
use crate::*;
#[cfg(not(feature = "alloc"))]
use sbor::rust::fmt;
use sbor::rust::prelude::*;
use sbor::*;
use utils::copy_u8_array;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternalRef(pub [u8; OBJECT_ID_LENGTH]);

impl InternalRef {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl TryFrom<&[u8]> for InternalRef {
    type Error = ParseReferenceError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        match slice.len() {
            OBJECT_ID_LENGTH => Ok(Self(copy_u8_array(slice))),
            _ => Err(ParseReferenceError::InvalidLength(slice.len())),
        }
    }
}

//========
// error
//========

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseReferenceError {
    InvalidLength(usize),
}

#[cfg(not(feature = "alloc"))]
impl std::error::Error for ParseReferenceError {}

#[cfg(not(feature = "alloc"))]
impl fmt::Display for ParseReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

//========
// binary
//========

well_known_scrypto_custom_type!(
    InternalRef,
    ScryptoCustomValueKind::Reference,
    Type::Reference,
    OBJECT_ID_LENGTH,
    REFERENCE_ID
);
