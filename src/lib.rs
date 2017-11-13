#[macro_use]
mod field;
#[macro_use]
mod group;
mod scalar;
mod ecmult;
mod ecdsa;

use field::Field;
use group::Affine;

pub struct PublicKey([u8; 64]);
pub struct Signature([u8; 64]);

pub fn public_key_load(pubkey: &PublicKey) -> Affine {
    let mut ge = Affine::default();
    let (mut x, mut y) = (Field::default(), Field::default());

    let mut data = [0u8; 32];
    for i in 0..32 {
        data[i] = pubkey.0[i];
    }
    x.set_b32(data.clone());
    for i in 0..32 {
        data[i] = pubkey.0[i+32];
    }
    x.set_b32(data.clone());

    ge.set_xy(&x, &y);
    assert!(!ge.x.is_zero());

    ge
}
