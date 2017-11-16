#![no_std]

#[macro_use]
mod field;
#[macro_use]
mod group;
mod scalar;
mod ecmult;
mod ecdsa;

pub use field::Field;
pub use group::{Affine, Jacobian, AffineStorage, AFFINE_G,
                AFFINE_INFINITY, JACOBIAN_INFINITY, CURVE_B,
                set_table_gej_var, globalz_set_table_gej};
pub use scalar::Scalar;

pub use ecmult::{ECMultContext, ECMultGenContext,
                 ECMULT_CONTEXT, ECMULT_GEN_CONTEXT, odd_multiples_table,
                 WINDOW_A, WINDOW_G, ECMULT_TABLE_SIZE_A, ECMULT_TABLE_SIZE_G};

pub const TAG_PUBKEY_EVEN: u8 = 0x02;
pub const TAG_PUBKEY_ODD: u8 = 0x03;
pub const TAG_PUBKEY_UNCOMPRESSED: u8 = 0x04;
pub const TAG_PUBKEY_HYBRID_EVEN: u8 = 0x06;
pub const TAG_PUBKEY_HYBRID_ODD: u8 = 0x07;

#[derive(Debug, Clone)]
pub struct PublicKey(pub Affine);
#[derive(Debug, Clone)]
pub struct SecretKey(pub Scalar);
#[derive(Debug, Clone)]
pub struct Signature {
    pub r: Scalar,
    pub s: Scalar
}
#[derive(Debug, Clone)]
pub struct RecoveryId(pub u8);
#[derive(Debug, Clone)]
pub struct Message(pub Scalar);

impl PublicKey {
    pub fn from_secret_key(seckey: &SecretKey) -> PublicKey {
        let mut pj = Jacobian::default();
        ECMULT_GEN_CONTEXT.ecmult_gen(&mut pj, &seckey.0);
        let mut p = Affine::default();
        p.set_gej(&pj);
        PublicKey(p)
    }

    pub fn parse(p: &[u8; 65]) -> Option<PublicKey> {
        use {TAG_PUBKEY_HYBRID_EVEN, TAG_PUBKEY_HYBRID_ODD};

        if !(p[0] == 0x04 || p[0] == 0x06 || p[0] == 0x07) {
            return None;
        }
        let mut x = Field::default();
        let mut y = Field::default();
        let mut data = [0u8; 32];
        for i in 0..32 {
            data[i] = p[i+1];
        }
        if !x.set_b32(&data) {
            return None;
        }
        for i in 0..32 {
            data[i] = p[i+33];
        }
        if !y.set_b32(&data) {
            return None;
        }
        let mut elem = Affine::default();
        elem.set_xy(&x, &y);
        if (p[0] == TAG_PUBKEY_HYBRID_EVEN || p[0] == TAG_PUBKEY_HYBRID_ODD) &&
            (y.is_odd() != (p[0] == TAG_PUBKEY_HYBRID_ODD))
        {
            return None;
        }
        if elem.is_valid_var() {
            return Some(PublicKey(elem));
        } else {
            return None;
        }
    }

    pub fn serialize(&self) -> Option<[u8; 65]> {
        if self.0.is_infinity() {
            return None;
        }

        let mut ret = [0u8; 65];
        let mut elem = self.0.clone();

        elem.x.normalize_var();
        elem.y.normalize_var();
        let d = elem.x.b32();
        for i in 0..32 {
            ret[1+i] = d[i];
        }
        let d = elem.y.b32();
        for i in 0..32 {
            ret[33+i] = d[i];
        }
        ret[0] = TAG_PUBKEY_UNCOMPRESSED;

        Some(ret)
    }
}

impl SecretKey {
    pub fn parse(p: &[u8; 32]) -> Option<SecretKey> {
        let mut elem = Scalar::default();
        if !elem.set_b32(p) && !elem.is_zero() {
            Some(SecretKey(elem))
        } else {
            None
        }
    }

    pub fn serialize(&self) -> [u8; 32] {
        self.0.b32()
    }
}

impl Signature {
    pub fn parse(p: &[u8; 64]) -> Signature {
        let mut r = Scalar::default();
        let mut s = Scalar::default();

        let mut data = [0u8; 32];
        for i in 0..32 {
            data[i] = p[i];
        }
        r.set_b32(&data);
        for i in 0..32 {
            data[i] = p[i+32];
        }
        s.set_b32(&data);

        Signature { r, s }
    }

    pub fn serialize(&self) -> [u8; 64] {
        let mut ret = [0u8; 64];

        let ra = self.r.b32();
        for i in 0..32 {
            ret[i] = ra[i];
        }
        let sa = self.s.b32();
        for i in 0..32 {
            ret[i+32] = sa[i];
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

pub fn verify(signature: &Signature, pubkey: &PublicKey, message: &Message) -> bool {
    ECMULT_CONTEXT.verify_raw(&signature.r, &signature.s, &pubkey.0, &message.0)
}

pub fn recover(signature: &Signature, recovery_id: &RecoveryId, message: &Message) -> Option<PublicKey> {
    ECMULT_CONTEXT.recover_raw(&signature.r, &signature.s, recovery_id.0, &message.0).map(|v| PublicKey(v))
}
