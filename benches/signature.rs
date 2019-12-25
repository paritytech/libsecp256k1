#![feature(test)]

extern crate rand;
extern crate secp256k1;
extern crate secp256k1_test;
extern crate test;

use rand::thread_rng;
use secp256k1::Signature;
use secp256k1_test::{Message as SecpMessage, Secp256k1};
use test::Bencher;

#[bench]
fn bench_signature_parse(b: &mut Bencher) {
    let secp256k1 = Secp256k1::new();
    let message_arr = [5u8; 32];
    let (privkey, _) = secp256k1.generate_keypair(&mut thread_rng()).unwrap();
    let message = SecpMessage::from_slice(&message_arr).unwrap();
    let signature = secp256k1.sign(&message, &privkey).unwrap();
    let signature_arr = signature.serialize_compact(&secp256k1);
    assert!(signature_arr.len() == 64);
    let mut signature_a = [0u8; 64];
    signature_a.copy_from_slice(&signature_arr[0..64]);

    b.iter(|| {
        let _signature = Signature::parse(&signature_a);
    });
}

#[bench]
fn bench_signature_serialize(b: &mut Bencher) {
    let secp256k1 = Secp256k1::new();
    let message_arr = [5u8; 32];
    let (privkey, _) = secp256k1.generate_keypair(&mut thread_rng()).unwrap();
    let message = SecpMessage::from_slice(&message_arr).unwrap();
    let signature = secp256k1.sign(&message, &privkey).unwrap();
    let signature_arr = signature.serialize_compact(&secp256k1);
    assert!(signature_arr.len() == 64);
    let mut signature_a = [0u8; 64];
    signature_a.copy_from_slice(&signature_arr[0..64]);
    let signature = Signature::parse(&signature_a);

    b.iter(|| {
        let _serialized = signature.serialize();
    });
}
