//! Pure Rust implementation of the secp256k1 curve and fast ECDSA
//! signatures. The secp256k1 curve is used excusively in Bitcoin and
//! Ethereum alike cryptocurrencies.

#![deny(unused_import_braces, unused_imports,
        unused_comparisons, unused_must_use,
        unused_variables, non_shorthand_field_patterns,
        unreachable_code, unused_parens)]

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod field;
#[macro_use]
mod group;
mod scalar;
mod error;
mod der;
mod ecmult;
mod ecdsa;
mod ecdh;
#[cfg(not(feature = "noconst"))]
mod context;

#[macro_use]
extern crate alloc;

pub use crate::error::Error;

/// Curve related structs.
pub mod curve {
    pub use crate::field::{Field, FieldStorage};
    pub use crate::group::{Affine, Jacobian, AffineStorage, AFFINE_G, CURVE_B};
    pub use crate::scalar::Scalar;

    pub use crate::ecmult::{ECMultContext, ECMultGenContext};
    #[cfg(not(feature = "noconst"))]
    pub use crate::context::{ECMULT_CONTEXT, ECMULT_GEN_CONTEXT};
}

/// Utilities to manipulate the secp256k1 curve parameters.
pub mod util {
    pub const TAG_PUBKEY_EVEN: u8 = 0x02;
    pub const TAG_PUBKEY_ODD: u8 = 0x03;
    pub const TAG_PUBKEY_FULL: u8 = 0x04;
    pub const TAG_PUBKEY_HYBRID_EVEN: u8 = 0x06;
    pub const TAG_PUBKEY_HYBRID_ODD: u8 = 0x07;

    pub const MESSAGE_SIZE: usize = 32;
    pub const SECRET_KEY_SIZE: usize = 32;
    pub const RAW_PUBLIC_KEY_SIZE: usize = 64;
    pub const FULL_PUBLIC_KEY_SIZE: usize = 65;
    pub const COMPRESSED_PUBLIC_KEY_SIZE: usize = 33;
    pub const SIGNATURE_SIZE: usize = 64;
    pub const DER_MAX_SIGNATURE_SIZE: usize = 72;

    pub use crate::group::{AFFINE_INFINITY, JACOBIAN_INFINITY,
                           set_table_gej_var, globalz_set_table_gej};
    pub use crate::ecmult::{WINDOW_A, WINDOW_G, ECMULT_TABLE_SIZE_A, ECMULT_TABLE_SIZE_G,
                            odd_multiples_table};

    pub use crate::der::{Decoder, SignatureArray};
}

#[cfg(not(feature = "noconst"))]
pub use crate::context::{
    PublicKey, SecretKey, Signature, RecoveryId, Message, SharedSecret,
    PublicKeyFormat, verify, recover,
};
#[cfg(all(not(feature = "noconst"), feature = "hmac"))]
pub use crate::context::sign;

#[cfg(test)]
mod tests {
    use crate::SecretKey;
    use hex_literal::hex;

    #[test]
    fn secret_key_inverse_is_sane() {
        let sk = SecretKey::parse(&[1; 32]).unwrap();
        let inv = sk.inv();
        let invinv = inv.inv();
        assert_eq!(sk, invinv);
        // Check that the inverse of `[1; 32]` is same as rust-secp256k1
        assert_eq!(inv, SecretKey::parse(&hex!("1536f1d756d1abf83aaf173bc5ee3fc487c93010f18624d80bd6d4038fadd59e")).unwrap())
    }
}
