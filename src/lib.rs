#[cfg(test)]
extern crate hexutil;

#[macro_use]
mod field;
#[macro_use]
mod group;
mod scalar;
mod ecmult;
mod ecdsa;

pub use field::Field;
pub use group::Affine;
pub use scalar::Scalar;

pub use ecmult::ECMultContext;

pub const TAG_PUBKEY_EVEN: u8 = 0x02;
pub const TAG_PUBKEY_ODD: u8 = 0x03;
pub const TAG_PUBKEY_UNCOMPRESSED: u8 = 0x04;
pub const TAG_PUBKEY_HYBRID_EVEN: u8 = 0x06;
pub const TAG_PUBKEY_HYBRID_ODD: u8 = 0x07;

pub struct PublicKey(pub [u8; 64]);
pub struct Signature(pub [u8; 64]);

impl PublicKey {
    pub fn load(&self) -> Affine {
        let mut ge = Affine::default();
        let (mut x, mut y) = (Field::default(), Field::default());

        let mut data = [0u8; 32];
        for i in 0..32 {
            data[i] = self.0[i];
        }
        x.set_b32(&data);
        for i in 0..32 {
            data[i] = self.0[i+32];
        }
        x.set_b32(&data);

        ge.set_xy(&x, &y);
        assert!(!ge.x.is_zero());

        ge
    }
}

impl Signature {
    pub fn load(&self) -> (Scalar, Scalar) {
        let mut r = Scalar::default();
        let mut s = Scalar::default();

        let mut data = [0u8; 32];
        for i in 0..32 {
            data[i] = self.0[i];
        }
        r.set_b32(&data);
        for i in 0..32 {
            data[i] = self.0[i+32];
        }
        s.set_b32(&data);

        (r, s)
    }
}
