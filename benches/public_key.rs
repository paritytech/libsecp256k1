#![feature(test)]

extern crate test;
extern crate secp256k1;
extern crate secp256k1_test;
extern crate rand;

use test::Bencher;
use secp256k1::PublicKey;
use secp256k1_test::Secp256k1;
use rand::thread_rng;

#[bench]
fn bench_public_key_parse(b: &mut Bencher) {
    let secp256k1 = Secp256k1::new();
    let (_, secp_pubkey) = secp256k1.generate_keypair(&mut thread_rng()).unwrap();
    let pubkey_arr = secp_pubkey.serialize_vec(&secp256k1, false);
    assert!(pubkey_arr.len() == 65);
    let mut pubkey_a = [0u8; 65];
    pubkey_a[0..65].copy_from_slice(&pubkey_arr[0..65]);
    b.iter(|| {
        let _pubkey = PublicKey::parse(&pubkey_a).unwrap();
    });
}

#[bench]
fn bench_public_key_serialize(b: &mut Bencher) {
    let secp256k1 = Secp256k1::new();
    let (_, secp_pubkey) = secp256k1.generate_keypair(&mut thread_rng()).unwrap();
    let pubkey_arr = secp_pubkey.serialize_vec(&secp256k1, false);
    assert!(pubkey_arr.len() == 65);
    let mut pubkey_a = [0u8; 65];
    pubkey_a[0..65].copy_from_slice(&pubkey_arr[0..65]);
    let pubkey = PublicKey::parse(&pubkey_a).unwrap();
    b.iter(|| {
        let _serialized = pubkey.serialize();
    });
}

#[bench]
fn bench_public_key_serialize_compressed(b: &mut Bencher) {
    let secp256k1 = Secp256k1::new();
    let (_, secp_pubkey) = secp256k1.generate_keypair(&mut thread_rng()).unwrap();
    let pubkey_arr = secp_pubkey.serialize_vec(&secp256k1, false);
    assert!(pubkey_arr.len() == 65);
    let mut pubkey_a = [0u8; 65];
    pubkey_a[0..65].copy_from_slice(&pubkey_arr[0..65]);
    let pubkey = PublicKey::parse(&pubkey_a).unwrap();
    b.iter(|| {
		let _serialized = pubkey.serialize_compressed();
    });
}
