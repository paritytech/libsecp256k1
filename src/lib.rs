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
mod ecmult;
mod ecdsa;
mod ecdh;
mod error;
mod der;

use hmac_drbg::HmacDRBG;
use sha2::Sha256;
use typenum::U32;
use arrayref::{array_ref, array_mut_ref};
use rand::Rng;

use crate::field::Field;
use crate::group::{Affine, Jacobian};
use crate::scalar::Scalar;
use crate::ecmult::{ECMULT_CONTEXT, ECMULT_GEN_CONTEXT};

pub use crate::error::Error;

/// Curve related structs.
pub mod curve {
    pub use crate::field::Field;
    pub use crate::group::{Affine, Jacobian, AffineStorage, AFFINE_G, CURVE_B};
    pub use crate::scalar::Scalar;

    pub use crate::ecmult::{ECMultContext, ECMultGenContext,
                            ECMULT_CONTEXT, ECMULT_GEN_CONTEXT};
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

    pub use crate::der::SignatureArray;
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

/// Format for public key parsing.
pub enum PublicKeyFormat {
    /// Compressed public key, 33 bytes.
    Compressed,
    /// Full length public key, 65 bytes.
    Full,
    /// Raw public key, 64 bytes.
    Raw,
}

impl PublicKey {
    pub fn from_secret_key(seckey: &SecretKey) -> PublicKey {
        let mut pj = Jacobian::default();
        ECMULT_GEN_CONTEXT.ecmult_gen(&mut pj, &seckey.0);
        let mut p = Affine::default();
        p.set_gej(&pj);
        PublicKey(p)
    }

    pub fn parse_slice(p: &[u8], format: Option<PublicKeyFormat>) -> Result<PublicKey, Error> {
        let format = match (p.len(), format) {
            (util::FULL_PUBLIC_KEY_SIZE, None) |
            (util::FULL_PUBLIC_KEY_SIZE, Some(PublicKeyFormat::Full)) =>
                PublicKeyFormat::Full,
            (util::COMPRESSED_PUBLIC_KEY_SIZE, None) |
            (util::COMPRESSED_PUBLIC_KEY_SIZE, Some(PublicKeyFormat::Compressed)) =>
                PublicKeyFormat::Compressed,
            (util::RAW_PUBLIC_KEY_SIZE, None) |
            (util::RAW_PUBLIC_KEY_SIZE, Some(PublicKeyFormat::Raw)) =>
                PublicKeyFormat::Raw,
            _ => return Err(Error::InvalidInputLength),
        };

        match format {
            PublicKeyFormat::Full => {
                let mut a = [0; util::FULL_PUBLIC_KEY_SIZE];
                a.copy_from_slice(p);
                Self::parse(&a)
            },
            PublicKeyFormat::Raw => {
                use util::TAG_PUBKEY_FULL;

                let mut a = [0; util::FULL_PUBLIC_KEY_SIZE];
                a[0] = TAG_PUBKEY_FULL;
                a[1..].copy_from_slice(p);
                Self::parse(&a)
            },
            PublicKeyFormat::Compressed => {
                let mut a = [0; util::COMPRESSED_PUBLIC_KEY_SIZE];
                a.copy_from_slice(p);
                Self::parse_compressed(&a)
            },
        }
    }

    pub fn parse(p: &[u8; util::FULL_PUBLIC_KEY_SIZE]) -> Result<PublicKey, Error> {
        use util::{TAG_PUBKEY_FULL, TAG_PUBKEY_HYBRID_EVEN, TAG_PUBKEY_HYBRID_ODD};

        if !(p[0] == TAG_PUBKEY_FULL || p[0] == TAG_PUBKEY_HYBRID_EVEN || p[0] == TAG_PUBKEY_HYBRID_ODD) {
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

    pub fn parse_compressed(p: &[u8; util::COMPRESSED_PUBLIC_KEY_SIZE]) -> Result<PublicKey, Error> {
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

    pub fn serialize(&self) -> [u8; util::FULL_PUBLIC_KEY_SIZE] {
        use util::TAG_PUBKEY_FULL;

        debug_assert!(!self.0.is_infinity());

        let mut ret = [0u8; 65];
        let mut elem = self.0.clone();

        elem.x.normalize_var();
        elem.y.normalize_var();
        elem.x.fill_b32(array_mut_ref!(ret, 1, 32));
        elem.y.fill_b32(array_mut_ref!(ret, 33, 32));
        ret[0] = TAG_PUBKEY_FULL;

        ret
    }

    pub fn serialize_compressed(&self) -> [u8; util::COMPRESSED_PUBLIC_KEY_SIZE] {
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

    pub fn tweak_add_assign(&mut self, tweak: &SecretKey) -> Result<(), Error> {
        let mut r = Jacobian::default();
        let a = Jacobian::from_ge(&self.0);
        let one = Scalar::from_int(1);
        ECMULT_CONTEXT.ecmult(&mut r, &a, &one, &tweak.0);

        if r.is_infinity() {
            return Err(Error::TweakOutOfRange);
        }

        self.0.set_gej(&r);
        Ok(())
    }

    pub fn tweak_mul_assign(&mut self, tweak: &SecretKey) -> Result<(), Error> {
        if tweak.0.is_zero() {
            return Err(Error::TweakOutOfRange);
        }

        let mut r = Jacobian::default();
        let zero = Scalar::from_int(0);
        let pt = Jacobian::from_ge(&self.0);
        ECMULT_CONTEXT.ecmult(&mut r, &pt, &tweak.0, &zero);

        self.0.set_gej(&r);
        Ok(())
    }

    pub fn combine(keys: &[PublicKey]) -> Result<Self, Error> {
        let mut qj = Jacobian::default();
        qj.set_infinity();

        for key in keys {
            qj = qj.add_ge(&key.0);
        }

        if qj.is_infinity() {
            return Err(Error::InvalidPublicKey);
        }

        let q = Affine::from_gej(&qj);
        Ok(PublicKey(q))
    }
}

impl Into<Affine> for PublicKey {
    fn into(self) -> Affine {
        self.0
    }
}

impl SecretKey {
    pub fn parse(p: &[u8; util::SECRET_KEY_SIZE]) -> Result<SecretKey, Error> {
        let mut elem = Scalar::default();
        if !elem.set_b32(p) && !elem.is_zero() {
            Ok(SecretKey(elem))
        } else {
            Err(Error::InvalidSecretKey)
        }
    }

    pub fn parse_slice(p: &[u8]) -> Result<SecretKey, Error> {
        if p.len() != util::SECRET_KEY_SIZE {
            return Err(Error::InvalidInputLength);
        }

        let mut a = [0; 32];
        a.copy_from_slice(p);
        Self::parse(&a)
    }

    pub fn random<R: Rng>(rng: &mut R) -> SecretKey {
        loop {
            let mut ret = [0u8; util::SECRET_KEY_SIZE];
            rng.fill_bytes(&mut ret);

            match Self::parse(&ret) {
                Ok(key) => return key,
                Err(_) => (),
            }
        }
    }

    pub fn serialize(&self) -> [u8; util::SECRET_KEY_SIZE] {
        self.0.b32()
    }

    pub fn tweak_add_assign(&mut self, tweak: &SecretKey) -> Result<(), Error> {
        let v = &self.0 + &tweak.0;
        if v.is_zero() {
            return Err(Error::TweakOutOfRange);
        }
        self.0 = v;
        Ok(())
    }

    pub fn tweak_mul_assign(&mut self, tweak: &SecretKey) -> Result<(), Error> {
        if tweak.0.is_zero() {
            return Err(Error::TweakOutOfRange);
        }

        self.0 *= &tweak.0;
        Ok(())
    }

    pub fn inv(&self) -> Self {
        SecretKey(self.0.inv())
    }
}

impl Default for SecretKey {
    fn default() -> SecretKey {
        let mut elem = Scalar::default();
        let overflowed = elem.set_b32(
            &[0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
		      0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
              0x00,0x00,0x00,0x00,0x00,0x01]
        );
        debug_assert!(!overflowed);
        debug_assert!(!elem.is_zero());
        SecretKey(elem)
    }
}

impl Into<Scalar> for SecretKey {
    fn into(self) -> Scalar {
        self.0.clone()
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        self.0.clear();
    }
}

impl Signature {
    pub fn parse(p: &[u8; util::SIGNATURE_SIZE]) -> Signature {
        let mut r = Scalar::default();
        let mut s = Scalar::default();

        // Okay for signature to overflow
        let _ = r.set_b32(array_ref!(p, 0, 32));
        let _ = s.set_b32(array_ref!(p, 32, 32));

        Signature { r, s }
    }

    pub fn parse_slice(p: &[u8]) -> Result<Signature, Error> {
        if p.len() != util::SIGNATURE_SIZE {
            return Err(Error::InvalidInputLength);
        }

        let mut a = [0; util::SIGNATURE_SIZE];
        a.copy_from_slice(p);
        Ok(Self::parse(&a))
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

    /// Converts a "lax DER"-encoded byte slice to a signature. This is basically
    /// only useful for validating signatures in the Bitcoin blockchain from before
    /// 2016. It should never be used in new applications. This library does not
    /// support serializing to this "format"
    pub fn parse_der_lax(p: &[u8]) -> Result<Signature, Error> {
        let mut decoder = der::Decoder::new(p);

        decoder.read_constructed_sequence()?;
        decoder.read_seq_len_lax()?;

        let r = decoder.read_integer_lax()?;
        let s = decoder.read_integer_lax()?;

        Ok(Signature { r, s })
    }

    /// Normalizes a signature to a "low S" form. In ECDSA, signatures are
    /// of the form (r, s) where r and s are numbers lying in some finite
    /// field. The verification equation will pass for (r, s) iff it passes
    /// for (r, -s), so it is possible to ``modify'' signatures in transit
    /// by flipping the sign of s. This does not constitute a forgery since
    /// the signed message still cannot be changed, but for some applications,
    /// changing even the signature itself can be a problem. Such applications
    /// require a "strong signature". It is believed that ECDSA is a strong
    /// signature except for this ambiguity in the sign of s, so to accommodate
    /// these applications libsecp256k1 will only accept signatures for which
    /// s is in the lower half of the field range. This eliminates the
    /// ambiguity.
    ///
    /// However, for some systems, signatures with high s-values are considered
    /// valid. (For example, parsing the historic Bitcoin blockchain requires
    /// this.) For these applications we provide this normalization function,
    /// which ensures that the s value lies in the lower half of its range.
    pub fn normalize_s(&mut self) {
        if self.s.is_high() {
            let s = self.s.clone();
            self.s.neg_in_place(&s);
        }
    }


    pub fn serialize(&self) -> [u8; util::SIGNATURE_SIZE] {
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
    pub fn parse(p: &[u8; util::MESSAGE_SIZE]) -> Message {
        let mut m = Scalar::default();

        // Okay for message to overflow.
        let _ = m.set_b32(p);

        Message(m)
    }

    pub fn parse_slice(p: &[u8]) -> Result<Message, Error> {
        if p.len() != util::MESSAGE_SIZE {
            return Err(Error::InvalidInputLength);
        }

        let mut a = [0; util::MESSAGE_SIZE];
        a.copy_from_slice(p);
        Ok(Self::parse(&a))
    }

    pub fn serialize(&self) -> [u8; util::MESSAGE_SIZE] {
        self.0.b32()
    }
}

impl RecoveryId {
    /// Parse recovery ID starting with 0.
    pub fn parse(p: u8) -> Result<RecoveryId, Error> {
        if p < 4 {
            Ok(RecoveryId(p))
        } else {
            Err(Error::InvalidRecoveryId)
        }
    }

    /// Parse recovery ID as Ethereum RPC format, starting with 27.
    pub fn parse_rpc(p: u8) -> Result<RecoveryId, Error> {
        if p >= 27 && p < 27 + 4 {
            RecoveryId::parse(p - 27)
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

impl Drop for SharedSecret {
    fn drop(&mut self) {
         unsafe {
            core::ptr::write_volatile(&mut self.0, [0u8; 32]);
        }
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
pub fn sign(message: &Message, seckey: &SecretKey) -> (Signature, RecoveryId) {
    let seckey_b32 = seckey.0.b32();
    let message_b32 = message.0.b32();

    let mut drbg = HmacDRBG::<Sha256>::new(&seckey_b32, &message_b32, &[]);
    let mut nonce = Scalar::default();
    let mut overflow;

    let result;
    loop {
        let generated = drbg.generate::<U32>(None);
        overflow = nonce.set_b32(array_ref!(generated, 0, 32));

        if !overflow && !nonce.is_zero() {
            match ECMULT_GEN_CONTEXT.sign_raw(&seckey.0, &message.0, &nonce) {
                Ok(val) => {
                    result = val;
                    break
                },
                Err(_) => (),
            }
        }
    }

    #[allow(unused_assignments)]
    {
        nonce = Scalar::default();
    }
    let (sigr, sigs, recid) = result;

    (Signature {
        r: sigr,
        s: sigs,
    }, RecoveryId(recid))
}
