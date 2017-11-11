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
}
