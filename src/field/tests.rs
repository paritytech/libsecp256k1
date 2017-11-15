use super::*;
use hexutil::*;

fn from_hex(s: &str) -> Field {
    let s = read_hex(s).unwrap();
    let mut a = [0u8; 32];
    for i in (0..s.len()).rev() {
        a[(32-s.len())+i] = s[i];
    }
    let mut f = Field::default();
    debug_assert!(f.set_b32(&a));
    f
}

#[test]
fn test_normalize() {
    let tests = [
        ([0x00000005, 0, 0, 0, 0, 0, 0, 0, 0, 0],
         [0x00000005, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        ([0x04000000, 0x0, 0, 0, 0, 0, 0, 0, 0, 0],
         [0x00000000, 0x1, 0, 0, 0, 0, 0, 0, 0, 0]),
        ([0x04000001, 0x0, 0, 0, 0, 0, 0, 0, 0, 0],
         [0x00000001, 0x1, 0, 0, 0, 0, 0, 0, 0, 0]),
        ([0xffffffff, 0x00, 0, 0, 0, 0, 0, 0, 0, 0],
         [0x03ffffff, 0x3f, 0, 0, 0, 0, 0, 0, 0, 0]),
        // 2^32
		([0x04000000, 0x3f, 0, 0, 0, 0, 0, 0, 0, 0],
         [0x00000000, 0x40, 0, 0, 0, 0, 0, 0, 0, 0]),
		// 2^32 + 1
		([0x04000001, 0x3f, 0, 0, 0, 0, 0, 0, 0, 0],
         [0x00000001, 0x40, 0, 0, 0, 0, 0, 0, 0, 0]),
		// 2^64 - 1
		([0xffffffff, 0xffffffc0, 0xfc0, 0, 0, 0, 0, 0, 0, 0],
         [0x03ffffff, 0x03ffffff, 0xfff, 0, 0, 0, 0, 0, 0, 0]),
		// 2^64
		([0x04000000, 0x03ffffff, 0x0fff, 0, 0, 0, 0, 0, 0, 0],
         [0x00000000, 0x00000000, 0x1000, 0, 0, 0, 0, 0, 0, 0]),
		// 2^64 + 1
		([0x04000001, 0x03ffffff, 0x0fff, 0, 0, 0, 0, 0, 0, 0],
         [0x00000001, 0x00000000, 0x1000, 0, 0, 0, 0, 0, 0, 0]),
		// 2^96 - 1
		([0xffffffff, 0xffffffc0, 0xffffffc0, 0x3ffc0, 0, 0, 0, 0, 0, 0],
         [0x03ffffff, 0x03ffffff, 0x03ffffff, 0x3ffff, 0, 0, 0, 0, 0, 0]),
		// 2^96
		([0x04000000, 0x03ffffff, 0x03ffffff, 0x3ffff, 0, 0, 0, 0, 0, 0],
         [0x00000000, 0x00000000, 0x00000000, 0x40000, 0, 0, 0, 0, 0, 0]),
		// 2^128 - 1
		([0xffffffff, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffc0, 0, 0, 0, 0, 0],
         [0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0xffffff, 0, 0, 0, 0, 0]),
		// 2^128
		([0x04000000, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x0ffffff, 0, 0, 0, 0, 0],
         [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x1000000, 0, 0, 0, 0, 0]),
		// 2^256 - 4294968273 (secp256k1 prime)
		([0xfffffc2f, 0xffffff80, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0x3fffc0],
         [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x000000]),
		// Prime larger than P where both first and second words are larger
		// than P's first and second words
		([0xfffffc30, 0xffffff86, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0x3fffc0],
         [0x00000001, 0x00000006, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x000000]),
		// Prime larger than P where only the second word is larger
		// than P's second words.
		([0xfffffc2a, 0xffffff87, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0x3fffc0],
         [0x03fffffb, 0x00000006, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x000000]),
		// 2^256 - 1
		([0xffffffff, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0xffffffc0, 0x3fffc0],
         [0x000003d0, 0x00000040, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x000000]),
		// Prime with field representation such that the initial
		// reduction does not result in a carry to bit 256.
		//
		// 2^256 - 4294968273 (secp256k1 prime)
		([0x03fffc2f, 0x03ffffbf, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x003fffff],
         [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]),
		// Prime larger than P that reduces to a value which is still
		// larger than P when it has a magnitude of 1 due to its first
		// word and does not result in a carry to bit 256.
		//
		// 2^256 - 4294968272 (secp256k1 prime + 1)
		([0x03fffc30, 0x03ffffbf, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x003fffff],
         [0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]),
		// Prime larger than P that reduces to a value which is still
		// larger than P when it has a magnitude of 1 due to its second
		// word and does not result in a carry to bit 256.
		//
		// 2^256 - 4227859409 (secp256k1 prime + 0x4000000)
		([0x03fffc2f, 0x03ffffc0, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x003fffff],
         [0x00000000, 0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]),
		// Prime larger than P that reduces to a value which is still
		// larger than P when it has a magnitude of 1 due to a carry to
		// bit 256, but would not be without the carry.  These values
		// come from the fact that P is 2^256 - 4294968273 and 977 is
		// the low order word in the internal field representation.
		//
		// 2^256 * 5 - ((4294968273 - (977+1)) * 4)
		([0x03ffffff, 0x03fffeff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x0013fffff],
         [0x00001314, 0x00000040, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x000000000]),
		// Prime larger than P that reduces to a value which is still
		// larger than P when it has a magnitude of 1 due to both a
		// carry to bit 256 and the first word.
		([0x03fffc30, 0x03ffffbf, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x07ffffff, 0x003fffff],
         [0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000001]),
		// Prime larger than P that reduces to a value which is still
		// larger than P when it has a magnitude of 1 due to both a
		// carry to bit 256 and the second word.
		//
		([0x03fffc2f, 0x03ffffc0, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x3ffffff, 0x07ffffff, 0x003fffff],
         [0x00000000, 0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x0000000, 0x00000000, 0x00000001]),
		// Prime larger than P that reduces to a value which is still
		// larger than P when it has a magnitude of 1 due to a carry to
		// bit 256 and the first and second words.
		//
		([0x03fffc30, 0x03ffffc0, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x03ffffff, 0x07ffffff, 0x003fffff],
         [0x00000001, 0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000001]),
    ];

    for test in &tests {
        let mut f1 = Field {
            n: test.0.clone(),
            magnitude: 1,
            normalized: false
        };
        let f2 = Field {
            n: test.1.clone(),
            magnitude: 1,
            normalized: true
        };
        f1.normalize_var();
        assert_eq!(f1, f2);
    }
}

#[test]
fn test_is_odd() {
    let tests = [
        (from_hex("0"), false),
        (from_hex("1"), true),
        (from_hex("2"), false),
        (from_hex("ffffffff"), true),
        (from_hex("fffffffffffffffe"), false),
    ];

    for test in &tests {
        assert_eq!(test.0.is_odd(), test.1);
    }
}
