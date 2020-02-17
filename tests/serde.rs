use secp256k1::*;

// Public key for secret key: "0101010101010101010101010101010101010101"
const DEBUG_PUBLIC_KEY: &str = "\"BBuExVZ7EmRAmV0+1aq6BWXXHhg0YEgZ/5wX9enV3QePcL6vj1iLVBUH/tamQsWrQt/fgSCn9jneUSLUemmo6NE=\"";

fn debug_public_key() -> PublicKey {
    let skey = SecretKey::parse(&[1u8; 32]).unwrap();
    PublicKey::from_secret_key(&skey)
}

#[test]
fn test_serialize_public_key() {
    let pkey = debug_public_key();
    let serialized_pkey = serde_json::to_string(&pkey).unwrap();
    assert_eq!(serialized_pkey, DEBUG_PUBLIC_KEY);
}

#[test]
fn test_deserialize_public_key() {
    let serialized_pkey = DEBUG_PUBLIC_KEY;
    let pkey: PublicKey = serde_json::from_str(&serialized_pkey).unwrap();
    assert_eq!(pkey, debug_public_key());
}
