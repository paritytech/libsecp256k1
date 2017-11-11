const SECP256K1_N_0: u32 = 0xD0364141;
const SECP256K1_N_1: u32 = 0xBFD25E8C;
const SECP256K1_N_2: u32 = 0xAF48A03B;
const SECP256K1_N_3: u32 = 0xBAAEDCE6;
const SECP256K1_N_4: u32 = 0xFFFFFFFE;
const SECP256K1_N_5: u32 = 0xFFFFFFFF;
const SECP256K1_N_6: u32 = 0xFFFFFFFF;
const SECP256K1_N_7: u32 = 0xFFFFFFFF;

const SECP256K1_N_C_0: u32 = !SECP256K1_N_0 + 1;
const SECP256K1_N_C_1: u32 = !SECP256K1_N_1;
const SECP256K1_N_C_2: u32 = !SECP256K1_N_2;
const SECP256K1_N_C_3: u32 = !SECP256K1_N_3;
const SECP256K1_N_C_4: u32 = 1;

const SECP256K1_N_H_0: u32 = 0x681B20A0;
const SECP256K1_N_H_1: u32 = 0xDFE92F46;
const SECP256K1_N_H_2: u32 = 0x57A4501D;
const SECP256K1_N_H_3: u32 = 0x5D576E73;
const SECP256K1_N_H_4: u32 = 0xFFFFFFFF;
const SECP256K1_N_H_5: u32 = 0xFFFFFFFF;
const SECP256K1_N_H_6: u32 = 0xFFFFFFFF;
const SECP256K1_N_H_7: u32 = 0x7FFFFFFF;

pub struct Scalar(pub [u32; 8]);

impl Scalar {
    /// Clear a scalar to prevent the leak of sensitive data.
    pub fn clear(&mut self) {
        self.0 = [0u32; 8];
    }

    /// Set a scalar to an unsigned integer.
    pub fn set_int(&mut self, v: u32) {
        self.0 = [v, 0, 0, 0, 0, 0, 0, 0];
    }

    /// Access bits from a scalar. All requested bits must belong to
    /// the same 32-bit limb.
    pub fn bits(&self, offset: usize, count: usize) -> u32 {
        debug_assert!((offset + count - 1) >> 5 == offset >> 5);
        (self.0[offset >> 5] >> (offset & 0x1F)) & ((1 << count) - 1)
    }

    /// Access bits from a scalar. Not constant time.
    pub fn bits_var(&self, offset: usize, count: usize) -> u32 {
        debug_assert!(count < 32);
        debug_assert!(offset + count <= 256);
        if (offset + count - 1) >> 5 == offset >> 5 {
            return self.bits(offset, count);
        } else {
            debug_assert!((offset >> 5) + 1 < 8);
            return ((self.0[offset >> 5] >> (offset & 0x1f)) | (self.0[(offset >> 5) + 1] << (32 - (offset & 0x1f)))) & ((1 << count) - 1);
        }
    }

    fn check_overflow(&self) -> bool {
        let mut yes: bool = false;
        let mut no: bool = false;
        no = no || (self.0[7] < SECP256K1_N_7); /* No need for a > check. */
        no = no || (self.0[6] < SECP256K1_N_6); /* No need for a > check. */
        no = no || (self.0[5] < SECP256K1_N_5); /* No need for a > check. */
        no = no || (self.0[4] < SECP256K1_N_4);
        yes = yes || ((self.0[4] > SECP256K1_N_4) && !no);
        no = no || ((self.0[3] < SECP256K1_N_3) && !yes);
        yes = yes || ((self.0[3] > SECP256K1_N_3) && !no);
        no = no || ((self.0[2] < SECP256K1_N_2) && !yes);
        yes = yes || ((self.0[2] > SECP256K1_N_2) && !no);
        no = no || ((self.0[1] < SECP256K1_N_1) && !yes);
        yes = yes || ((self.0[1] > SECP256K1_N_1) && !no);
        yes = yes || ((self.0[0] >= SECP256K1_N_0) && !no);
        return yes;
    }

    fn reduce(&mut self, overflow: bool) -> bool {
        let o: u64 = if overflow { 1 } else { 0 };
        let mut t: u64;
        t = (self.0[0] as u64) + o * (SECP256K1_N_C_0 as u64);
        self.0[0] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[1] as u64) + o * (SECP256K1_N_C_1 as u64);
        self.0[1] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[2] as u64) + o * (SECP256K1_N_C_2 as u64);
        self.0[2] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[3] as u64) + o * (SECP256K1_N_C_3 as u64);
        self.0[3] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[4] as u64) + o * (SECP256K1_N_C_4 as u64);
        self.0[4] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[5] as u64);
        self.0[5] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[6] as u64);
        self.0[6] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[7] as u64);
        self.0[7] = (t & 0xFFFFFFFF) as u32;
        overflow
    }

    /// Add two scalars together (modulo the group order). Returns
    /// whether it overflowed.
    pub fn add_in_place(&mut self, a: &Scalar, b: &Scalar) -> bool {
        let overflow: u64;
        let mut t: u64 = (a.0[0] as u64) + (b.0[0] as u64);
        self.0[0] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (a.0[1] as u64) + (b.0[1] as u64);
        self.0[1] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (a.0[2] as u64) + (b.0[2] as u64);
        self.0[2] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (a.0[3] as u64) + (b.0[3] as u64);
        self.0[3] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (a.0[4] as u64) + (b.0[4] as u64);
        self.0[4] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (a.0[5] as u64) + (b.0[5] as u64);
        self.0[5] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (a.0[6] as u64) + (b.0[6] as u64);
        self.0[6] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (a.0[7] as u64) + (b.0[7] as u64);
        self.0[7] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        overflow = t + if self.check_overflow() { 1 } else { 0 };
        debug_assert!(overflow == 0 || overflow == 1);
        self.reduce(overflow == 1);
        return overflow == 1;
    }

    /// Conditionally add a power of two to a scalar. The result is
    /// not allowed to overflow.
    pub fn cadd_bit(&mut self, mut bit: usize, flag: bool) {
        let mut t: u64;
        debug_assert!(bit < 256);
        bit += if flag { 0 } else { usize::max_value() } & 0x100;
        t = (self.0[0] as u64) + ((if ((bit >> 5) == 0) { 1 } else { 0 }) << (bit & 0x1F));
        self.0[0] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[1] as u64) + ((if ((bit >> 5) == 1) { 1 } else { 0 }) << (bit & 0x1F));
        self.0[1] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[2] as u64) + ((if ((bit >> 5) == 2) { 1 } else { 0 }) << (bit & 0x1F));
        self.0[2] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[3] as u64) + ((if ((bit >> 5) == 3) { 1 } else { 0 }) << (bit & 0x1F));
        self.0[3] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[4] as u64) + ((if ((bit >> 5) == 4) { 1 } else { 0 }) << (bit & 0x1F));
        self.0[4] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[5] as u64) + ((if ((bit >> 5) == 5) { 1 } else { 0 }) << (bit & 0x1F));
        self.0[5] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[6] as u64) + ((if ((bit >> 5) == 6) { 1 } else { 0 }) << (bit & 0x1F));
        self.0[6] = (t & 0xFFFFFFFF) as u32; t >>= 32;
        t += (self.0[7] as u64) + ((if ((bit >> 5) == 7) { 1 } else { 0 }) << (bit & 0x1F));
        self.0[7] = (t & 0xFFFFFFFF) as u32;
        debug_assert!((t >> 32) == 0);
        debug_assert!(!self.check_overflow());
    }

    /// Set a scalar from a big endian byte array.
    pub fn set_b32(&mut self, b32: [u8; 32]) -> bool {
        self.0[0] = (b32[31] as u32) | ((b32[30] as u32) << 8) | ((b32[29] as u32) << 16) | ((b32[28] as u32) << 24);
        self.0[1] = (b32[27] as u32) | ((b32[26] as u32) << 8) | ((b32[25] as u32) << 16) | ((b32[24] as u32) << 24);
        self.0[2] = (b32[23] as u32) | ((b32[22] as u32) << 8) | ((b32[21] as u32) << 16) | ((b32[20] as u32) << 24);
        self.0[3] = (b32[19] as u32) | ((b32[18] as u32) << 8) | ((b32[17] as u32) << 16) | ((b32[16] as u32) << 24);
        self.0[4] = (b32[15] as u32) | ((b32[14] as u32) << 8) | ((b32[13] as u32) << 16) | ((b32[12] as u32) << 24);
        self.0[5] = (b32[11] as u32) | ((b32[10] as u32) << 8) | ((b32[9] as u32) << 16) | ((b32[8] as u32) << 24);
        self.0[6] = (b32[7] as u32) | ((b32[6] as u32) << 8) | ((b32[5] as u32) << 16) | ((b32[4] as u32) << 24);
        self.0[7] = (b32[3] as u32) | ((b32[2] as u32) << 8) | ((b32[1] as u32) << 16) | ((b32[0] as u32) << 24);

        let overflow = self.check_overflow();
        self.reduce(overflow)
    }

    /// Convert a scalar to a byte array.
    pub fn b32(&self) -> [u8; 32] {
        let mut bin = [0u8; 32];
        bin[0] = (self.0[7] >> 24) as u8; bin[1] = (self.0[7] >> 16) as u8; bin[2] = (self.0[7] >> 8) as u8; bin[3] = (self.0[7]) as u8;
        bin[4] = (self.0[6] >> 24) as u8; bin[5] = (self.0[6] >> 16) as u8; bin[6] = (self.0[6] >> 8) as u8; bin[7] = (self.0[6]) as u8;
        bin[8] = (self.0[5] >> 24) as u8; bin[9] = (self.0[5] >> 16) as u8; bin[10] = (self.0[5] >> 8) as u8; bin[11] = (self.0[5]) as u8;
        bin[12] = (self.0[4] >> 24) as u8; bin[13] = (self.0[4] >> 16) as u8; bin[14] = (self.0[4] >> 8) as u8; bin[15] = (self.0[4]) as u8;
        bin[16] = (self.0[3] >> 24) as u8; bin[17] = (self.0[3] >> 16) as u8; bin[18] = (self.0[3] >> 8) as u8; bin[19] = (self.0[3]) as u8;
        bin[20] = (self.0[2] >> 24) as u8; bin[21] = (self.0[2] >> 16) as u8; bin[22] = (self.0[2] >> 8) as u8; bin[23] = (self.0[2]) as u8;
        bin[24] = (self.0[1] >> 24) as u8; bin[25] = (self.0[1] >> 16) as u8; bin[26] = (self.0[1] >> 8) as u8; bin[27] = (self.0[1]) as u8;
        bin[28] = (self.0[0] >> 24) as u8; bin[29] = (self.0[0] >> 16) as u8; bin[30] = (self.0[0] >> 8) as u8; bin[31] = (self.0[0]) as u8;
        bin
    }

    /// Check whether a scalar equals zero.
    pub fn is_zero(&self) -> bool {
        (self.0[0] | self.0[1] | self.0[2] | self.0[3] | self.0[4] | self.0[5] | self.0[6] | self.0[7]) == 0
    }

    /// Compute the complement of a scalar (modulo the group order).
    pub fn neg_in_place(&mut self, a: &Scalar) {
        let nonzero: u64 = 0xFFFFFFFF * if !self.is_zero() { 1 } else { 0 };
        let mut t: u64 = (!a.0[0]) as u64 + SECP256K1_N_0 as u64 + 1;
        self.0[0] = (t & nonzero) as u32; t >>= 32;
        t += (!a.0[1]) as u64 + SECP256K1_N_1 as u64;
        self.0[1] = (t & nonzero) as u32; t >>= 32;
        t += (!a.0[2]) as u64 + SECP256K1_N_2 as u64;
        self.0[2] = (t & nonzero) as u32; t >>= 32;
        t += (!a.0[3]) as u64 + SECP256K1_N_3 as u64;
        self.0[3] = (t & nonzero) as u32; t >>= 32;
        t += (!a.0[4]) as u64 + SECP256K1_N_4 as u64;
        self.0[4] = (t & nonzero) as u32; t >>= 32;
        t += (!a.0[5]) as u64 + SECP256K1_N_5 as u64;
        self.0[5] = (t & nonzero) as u32; t >>= 32;
        t += (!a.0[6]) as u64 + SECP256K1_N_6 as u64;
        self.0[6] = (t & nonzero) as u32; t >>= 32;
        t += (!a.0[7]) as u64 + SECP256K1_N_7 as u64;
        self.0[7] = (t & nonzero) as u32;
    }

    /// Check whether a scalar equals one.
    pub fn is_one(&self) -> bool {
        ((self.0[0] ^ 1) |  self.0[1] | self.0[2] | self.0[3] | self.0[4] | self.0[5] | self.0[6] | self.0[7]) == 0
    }
}
