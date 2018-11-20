//! Pure Rust implementation of the secp256k1 curve and fast ECDSA
//! signatures. The secp256k1 curve is used excusively in Bitcoin and
//! Ethereum alike cryptocurrencies.

#![deny(unused_import_braces, unused_imports,
        unused_comparisons, unused_must_use,
        unused_variables, non_shorthand_field_patterns,
        unreachable_code, unused_parens)]

#![no_std]
extern crate hmac_drbg;
extern crate typenum;
extern crate digest;
extern crate sha2;
extern crate rand;
#[macro_use]
extern crate arrayref;

#[macro_use]
mod field;
#[macro_use]
mod group;
mod scalar;
mod ecmult;
mod ecdsa;
mod ecdh;
mod error;
mod der;

use hmac_drbg::HmacDRBG;
use sha2::Sha256;
use typenum::U32;

use field::Field;
use group::{Affine, Jacobian};
use scalar::Scalar;

use ecmult::{ECMULT_CONTEXT, ECMULT_GEN_CONTEXT};

use rand::Rng;

pub use error::Error;

/// Curve related structs.
pub mod curve {
    pub use field::Field;
    pub use group::{Affine, Jacobian, AffineStorage, AFFINE_G, CURVE_B};
    pub use scalar::Scalar;

    pub use ecmult::{ECMultContext, ECMultGenContext,
                     ECMULT_CONTEXT, ECMULT_GEN_CONTEXT};
}

/// Utilities to manipulate the secp256k1 curve parameters.
pub mod util {
    pub const TAG_PUBKEY_EVEN: u8 = 0x02;
    pub const TAG_PUBKEY_ODD: u8 = 0x03;
    pub const TAG_PUBKEY_UNCOMPRESSED: u8 = 0x04;
    pub const TAG_PUBKEY_HYBRID_EVEN: u8 = 0x06;
    pub const TAG_PUBKEY_HYBRID_ODD: u8 = 0x07;

    pub use group::{AFFINE_INFINITY, JACOBIAN_INFINITY,
                    set_table_gej_var, globalz_set_table_gej};
    pub use ecmult::{WINDOW_A, WINDOW_G, ECMULT_TABLE_SIZE_A, ECMULT_TABLE_SIZE_G,
                     odd_multiples_table};

    pub use der::SignatureArray;
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// Public key on a secp256k1 curve.
pub struct PublicKey(Affine);
#[derive(Debug, Clone, Eq, PartialEq)]
/// Secret key (256-bit) on a secp256k1 curve.
pub struct SecretKey(Scalar);
#[derive(Debug, Clone, Eq, PartialEq)]
/// An ECDSA signature.
pub struct Signature {
    pub r: Scalar,
    pub s: Scalar
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// Tag used for public key recovery from signatures.
pub struct RecoveryId(u8);
#[derive(Debug, Clone, Eq, PartialEq)]
/// Hashed message input to an ECDSA signature.
pub struct Message(pub Scalar);
#[derive(Debug, Clone, Eq, PartialEq)]
/// Shared secret using ECDH.
pub struct SharedSecret([u8; 32]);

impl PublicKey {
    pub fn from_secret_key(seckey: &SecretKey) -> PublicKey {
        let mut pj = Jacobian::default();
        ECMULT_GEN_CONTEXT.ecmult_gen(&mut pj, &seckey.0);
        let mut p = Affine::default();
        p.set_gej(&pj);
        PublicKey(p)
    }

    pub fn parse_slice(p: &[u8], compressed: Option<bool>) -> Result<PublicKey, Error> {
        let compressed = match (p.len(), compressed) {
            (65, None) => false,
            (33, None) => true,
            (65, Some(false)) => false,
            (33, Some(true)) => true,
            _ => return Err(Error::InvalidInputLength),
        };

        if compressed {
            let mut a = [0; 33];
            a.copy_from_slice(p);
            Self::parse_compressed(&a)
        } else {
            let mut a = [0; 65];
            a.copy_from_slice(p);
            Self::parse(&a)
        }
    }

    pub fn parse(p: &[u8; 65]) -> Result<PublicKey, Error> {
        use util::{TAG_PUBKEY_UNCOMPRESSED, TAG_PUBKEY_HYBRID_EVEN, TAG_PUBKEY_HYBRID_ODD};

        if !(p[0] == TAG_PUBKEY_UNCOMPRESSED || p[0] == TAG_PUBKEY_HYBRID_EVEN || p[0] == TAG_PUBKEY_HYBRID_ODD) {
            return Err(Error::InvalidPublicKey);
        }
        let mut x = Field::default();
        let mut y = Field::default();
        if !x.set_b32(array_ref!(p, 1, 32)) {
            return Err(Error::InvalidPublicKey);
        }
        if !y.set_b32(array_ref!(p, 33, 32)) {
            return Err(Error::InvalidPublicKey);
        }
        let mut elem = Affine::default();
        elem.set_xy(&x, &y);
        if (p[0] == TAG_PUBKEY_HYBRID_EVEN || p[0] == TAG_PUBKEY_HYBRID_ODD) &&
            (y.is_odd() != (p[0] == TAG_PUBKEY_HYBRID_ODD))
        {
            return Err(Error::InvalidPublicKey);
        }
        if elem.is_infinity() {
            return Err(Error::InvalidPublicKey);
        }
        if elem.is_valid_var() {
            return Ok(PublicKey(elem));
        } else {
            return Err(Error::InvalidPublicKey);
        }
    }

    pub fn parse_compressed(p: &[u8; 33]) -> Result<PublicKey, Error> {
        use util::{TAG_PUBKEY_EVEN, TAG_PUBKEY_ODD};

        if !(p[0] == TAG_PUBKEY_EVEN || p[0] == TAG_PUBKEY_ODD) {
            return Err(Error::InvalidPublicKey);
        }
        let mut x = Field::default();
        if !x.set_b32(array_ref!(p, 1, 32)) {
            return Err(Error::InvalidPublicKey);
        }
        let mut elem = Affine::default();
        elem.set_xo_var(&x, p[0] == TAG_PUBKEY_ODD);
        if elem.is_infinity() {
            return Err(Error::InvalidPublicKey);
        }
        if elem.is_valid_var() {
            return Ok(PublicKey(elem));
        } else {
            return Err(Error::InvalidPublicKey);
        }
    }

    pub fn serialize(&self) -> [u8; 65] {
        use util::TAG_PUBKEY_UNCOMPRESSED;

        debug_assert!(!self.0.is_infinity());

        let mut ret = [0u8; 65];
        let mut elem = self.0.clone();

        elem.x.normalize_var();
        elem.y.normalize_var();
        elem.x.fill_b32(array_mut_ref!(ret, 1, 32));
        elem.y.fill_b32(array_mut_ref!(ret, 33, 32));
        ret[0] = TAG_PUBKEY_UNCOMPRESSED;

        ret
    }

    pub fn serialize_compressed(&self) -> [u8; 33] {
        use util::{TAG_PUBKEY_ODD, TAG_PUBKEY_EVEN};

        debug_assert!(!self.0.is_infinity());

        let mut ret = [0u8; 33];
        let mut elem = self.0.clone();

        elem.x.normalize_var();
        elem.y.normalize_var();
        elem.x.fill_b32(array_mut_ref!(ret, 1, 32));
        ret[0] = if elem.y.is_odd() {
            TAG_PUBKEY_ODD
        } else {
            TAG_PUBKEY_EVEN
        };

        ret
    }
}

impl Into<Affine> for PublicKey {
    fn into(self) -> Affine {
        self.0
    }
}

impl SecretKey {
    pub fn parse(p: &[u8; 32]) -> Result<SecretKey, Error> {
        let mut elem = Scalar::default();
        if !elem.set_b32(p) && !elem.is_zero() {
            Ok(SecretKey(elem))
        } else {
            Err(Error::InvalidSecretKey)
        }
    }

    pub fn random<R: Rng>(rng: &mut R) -> SecretKey {
        loop {
            let mut ret = [0u8; 32];
            rng.fill_bytes(&mut ret);

            match Self::parse(&ret) {
                Ok(key) => return key,
                Err(_) => (),
            }
        }
    }

    pub fn serialize(&self) -> [u8; 32] {
        self.0.b32()
    }
}

impl Into<Scalar> for SecretKey {
    fn into(self) -> Scalar {
        self.0
    }
}

impl Signature {
    pub fn parse(p: &[u8; 64]) -> Signature {
        let mut r = Scalar::default();
        let mut s = Scalar::default();

        r.set_b32(array_ref!(p, 0, 32));
        s.set_b32(array_ref!(p, 32, 32));

        Signature { r, s }
    }

    pub fn parse_der(p: &[u8]) -> Result<Signature, Error> {
        let mut decoder = der::Decoder::new(p);

        decoder.read_constructed_sequence()?;
        let rlen = decoder.read_len()?;

        if rlen != decoder.remaining_len() {
            return Err(Error::InvalidSignature);
        }

        let r = decoder.read_integer()?;
        let s = decoder.read_integer()?;

        if decoder.remaining_len() != 0 {
            return Err(Error::InvalidSignature);
        }

        Ok(Signature { r, s })
    }

    pub fn serialize(&self) -> [u8; 64] {
        let mut ret = [0u8; 64];
        self.r.fill_b32(array_mut_ref!(ret, 0, 32));
        self.s.fill_b32(array_mut_ref!(ret, 32, 32));
        ret
    }

    pub fn serialize_der(&self) -> der::SignatureArray {
        fn fill_scalar_with_leading_zero(scalar: &Scalar) -> [u8; 33] {
            let mut ret = [0u8; 33];
            scalar.fill_b32(array_mut_ref!(ret, 1, 32));
            ret
        }

        let r_full = fill_scalar_with_leading_zero(&self.r);
        let s_full = fill_scalar_with_leading_zero(&self.s);

        fn integer_slice(full: &[u8; 33]) -> &[u8] {
            let mut len = 33;
            while len > 1 &&
                full[full.len() - len] == 0 &&
                full[full.len() - len + 1] < 0x80
            {
                len -= 1;
            }
            &full[(full.len() - len)..]
        }

        let r = integer_slice(&r_full);
        let s = integer_slice(&s_full);

        let mut ret = der::SignatureArray::new(6 + r.len() + s.len());
        {
            let l = ret.as_mut();
            l[0] = 0x30;
            l[1] = 4 + r.len() as u8 + s.len() as u8;
            l[2] = 0x02;
            l[3] = r.len() as u8;
            l[4..(4 + r.len())].copy_from_slice(r);
            l[4 + r.len()] = 0x02;
            l[5 + r.len()] = s.len() as u8;
            l[(6 + r.len())..(6 + r.len() + s.len())].copy_from_slice(s);
        }

        ret
    }
}

impl Message {
    pub fn parse(p: &[u8; 32]) -> Message {
        let mut m = Scalar::default();
        m.set_b32(p);

        Message(m)
    }

    pub fn serialize(&self) -> [u8; 32] {
        self.0.b32()
    }
}

impl RecoveryId {
    pub fn parse(p: u8) -> Result<RecoveryId, Error> {
        if p < 4 {
            Ok(RecoveryId(p))
        } else {
            Err(Error::InvalidRecoveryId)
        }
    }

    pub fn serialize(&self) -> u8 {
        self.0
    }
}

impl Into<u8> for RecoveryId {
    fn into(self) -> u8 {
        self.0
    }
}

impl Into<i32> for RecoveryId {
    fn into(self) -> i32 {
        self.0 as i32
    }
}

impl SharedSecret {
    pub fn new(pubkey: &PublicKey, seckey: &SecretKey) -> Result<SharedSecret, Error> {
        let inner = match ECMULT_CONTEXT.ecdh_raw(&pubkey.0, &seckey.0) {
            Some(val) => val,
            None => return Err(Error::InvalidSecretKey),
        };

        Ok(SharedSecret(inner))
    }
}

impl AsRef<[u8]> for SharedSecret {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Check signature is a valid message signed by public key.
pub fn verify(message: &Message, signature: &Signature, pubkey: &PublicKey) -> bool {
    ECMULT_CONTEXT.verify_raw(&signature.r, &signature.s, &pubkey.0, &message.0)
}

/// Recover public key from a signed message.
pub fn recover(message: &Message, signature: &Signature, recovery_id: &RecoveryId) -> Result<PublicKey, Error> {
    ECMULT_CONTEXT.recover_raw(&signature.r, &signature.s, recovery_id.0, &message.0).map(|v| PublicKey(v))
}

/// Sign a message using the secret key.
pub fn sign(message: &Message, seckey: &SecretKey) -> Result<(Signature, RecoveryId), Error> {
    let seckey_b32 = seckey.0.b32();
    let message_b32 = message.0.b32();

    let mut drbg = HmacDRBG::<Sha256>::new(&seckey_b32, &message_b32, &[]);
    let generated = drbg.generate::<U32>(None);
    let mut nonce = Scalar::default();
    let mut overflow = nonce.set_b32(array_ref!(generated, 0, 32));

    while overflow || nonce.is_zero() {
        let generated = drbg.generate::<U32>(None);
        overflow = nonce.set_b32(array_ref!(generated, 0, 32));
    }

    let result = ECMULT_GEN_CONTEXT.sign_raw(&seckey.0, &message.0, &nonce);
    #[allow(unused_assignments)]
    {
        nonce = Scalar::default();
    }
    if let Ok((sigr, sigs, recid)) = result {
        return Ok((Signature {
            r: sigr,
            s: sigs,
        }, RecoveryId(recid)));
    } else {
        return Err(result.err().unwrap());
    }
}
