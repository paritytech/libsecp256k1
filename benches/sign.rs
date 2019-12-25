#![feature(test)]

extern crate rand;
extern crate secp256k1;
extern crate secp256k1_test;
extern crate test;
#[macro_use]
extern crate arrayref;

use rand::thread_rng;
use secp256k1::{sign, Message, SecretKey};
use secp256k1_test::Secp256k1;
use test::Bencher;

#[bench]
fn bench_sign_message(b: &mut Bencher) {
    let secp256k1 = Secp256k1::new();
    let message = Message::parse(&[5u8; 32]);
    let (secp_privkey, _) = secp256k1.generate_keypair(&mut thread_rng()).unwrap();
    let seckey = SecretKey::parse(array_ref!(secp_privkey, 0, 32)).unwrap();

    b.iter(|| {
        let _ = sign(&message, &seckey).unwrap();
    });
}
