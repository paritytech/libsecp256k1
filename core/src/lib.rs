//! Pure Rust implementation of the secp256k1 curve and fast ECDSA
//! signatures. The secp256k1 curve is used exclusively in Bitcoin and
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

extern crate alloc;

pub use crate::error::Error;

/// Curve related structs.
pub mod curve {
    pub use crate::field::{Field, FieldStorage};
    pub use crate::group::{Affine, Jacobian, AffineStorage, AFFINE_G, CURVE_B};
    pub use crate::scalar::Scalar;

    pub use crate::ecmult::{ECMultContext, ECMultGenContext};
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
