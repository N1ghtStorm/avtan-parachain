use codec::{Encode, Decode};
use sp_core::RuntimeDebug;
use sp_runtime::scale_info::TypeInfo;


#[derive(
    Encode,
    Decode,
    Eq,
    PartialEq,
    Copy,
    Clone,
    RuntimeDebug,
    PartialOrd,
    Ord,
    codec::MaxEncodedLen,
    TypeInfo,
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum TokenId {
    AVTN,
    Sora(SoraToken),
}

#[derive(
    Encode,
    Decode,
    Eq,
    PartialEq,
    Copy,
    Clone,
    RuntimeDebug,
    PartialOrd,
    Ord,
    codec::MaxEncodedLen,
    TypeInfo,
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum SoraToken {
    XOR,
    VAL,
    XSTUSD,
}