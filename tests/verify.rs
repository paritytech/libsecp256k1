extern crate secp256k1;
extern crate secp256k1_test;
extern crate rand;

use secp256k1::*;
use secp256k1_test::{Secp256k1, Message};
use rand::thread_rng;

#[test]
fn test_verify() {
    let secp256k1 = Secp256k1::new();

    let message_arr = [5u8; 32];
    let (privkey, pubkey) = secp256k1.generate_keypair(&mut thread_rng()).unwrap();
    let message = Message::from_slice(&message_arr).unwrap();
    let signature = secp256k1.sign(&message, &privkey).unwrap();

    let pubkey_arr = pubkey.serialize_vec(&secp256k1, false);
    assert!(pubkey_arr.len() == 65);
    let mut pubkey_a = [0u8; 65];
    for i in 0..65 {
        pubkey_a[i] = pubkey_arr[i];
    }

    let ctx_pubkey = PublicKey::parse(&pubkey_a).unwrap();
    let mut ctx_message = Scalar::default();
    ctx_message.set_b32(&message_arr);
    let signature_arr = signature.serialize_compact(&secp256k1);
    assert!(signature_arr.len() == 64);
    let mut signature_a = [0u8; 64];
    for i in 0..64 {
        signature_a[i] = signature_arr[i];
    }
    let ctx_sig = Signature::parse(&signature_a);

    secp256k1.verify(&message, &signature, &pubkey).unwrap();
    assert!(ECMULT_CONTEXT.verify_raw(&ctx_sig.r, &ctx_sig.s, &ctx_pubkey.0, &ctx_message));
    let mut f_ctx_sig = ctx_sig.clone();
    f_ctx_sig.r.set_int(0);
    if f_ctx_sig.r != ctx_sig.r {
        assert!(!ECMULT_CONTEXT.verify_raw(&f_ctx_sig.r, &ctx_sig.s, &ctx_pubkey.0, &ctx_message));
    }
    f_ctx_sig.r.set_int(1);
    if f_ctx_sig.r != ctx_sig.r {
        assert!(!ECMULT_CONTEXT.verify_raw(&f_ctx_sig.r, &ctx_sig.s, &ctx_pubkey.0, &ctx_message));
    }
}

#[test]
fn test_recover() {
    let secp256k1 = Secp256k1::new();

    let message_arr = [5u8; 32];
    let (privkey, pubkey) = secp256k1.generate_keypair(&mut thread_rng()).unwrap();
    let message = Message::from_slice(&message_arr).unwrap();
    let signature = secp256k1.sign_recoverable(&message, &privkey).unwrap();

    let pubkey_arr = pubkey.serialize_vec(&secp256k1, false);
    assert!(pubkey_arr.len() == 65);
    let mut pubkey_a = [0u8; 65];
    for i in 0..65 {
        pubkey_a[i] = pubkey_arr[i];
    }

    let mut ctx_message = Scalar::default();
    ctx_message.set_b32(&message_arr);
    let (rec_id, signature_arr) = signature.serialize_compact(&secp256k1);
    assert!(signature_arr.len() == 64);
    let mut signature_a = [0u8; 64];
    for i in 0..64 {
        signature_a[i] = signature_arr[i];
    }
    let ctx_sig = Signature::parse(&signature_a);

    secp256k1.recover(&message, &signature).unwrap();
    let ctx_pubkey = ECMULT_CONTEXT.recover_raw(&ctx_sig.r, &ctx_sig.s, rec_id.to_i32() as u8, &ctx_message).unwrap();
    let sp = PublicKey(ctx_pubkey).serialize().unwrap();

    let sps: &[u8] = &sp;
    let gps: &[u8] = &pubkey_a;
    assert_eq!(sps, gps);
}
