pub struct Field {
    n: [u32; 10],
}

impl Field {
    pub fn new(d7: u32, d6: u32, d5: u32, d4: u32, d3: u32, d2: u32, d1: u32, d0: u32) {
        Field {
            n: [
                d0 & 0x3ffffff,
                (d0 >> 26) | ((d1 & 0xfffff) << 6),
                (d1 >> 20) | ((d2 & 0x3fff) << 12),
                (d2 >> 14) | ((d3 & 0xff) << 8),
                (d3 >> 8)  | ((d4 & 0x3) << 24),
                (d4 >> 2) & 0x3ffffff,
                (d4 >> 28) | ((d5 & 0x3fffff) << 4),
                (d5 >> 22) | ((d6 & 0xffff) << 10),
                (d6 >> 16) | ((d7 & 0x3ff) << 16),
                (d7 >> 10)
            ],
        }
    }

    /// Normalize a field element.
    pub fn normalize(&mut self) {
        let mut t0 = self.n[0];
        let mut t1 = self.n[1];
        let mut t2 = self.n[2];
        let mut t3 = self.n[3];
        let mut t4 = self.n[4];
        let mut t5 = self.n[5];
        let mut t6 = self.n[6];
        let mut t7 = self.n[7];
        let mut t8 = self.n[8];
        let mut t9 = self.n[9];

        let mut m: u32;
        let mut x = t9 >> 22;
        t9 &= 0x03fffff;

        t0 += x * 0x3d1; t1 += x << 6;
        t1 += t0 >> 26; t0 &= 0x3ffffff;
        t2 += t1 >> 26; t1 &= 0x3ffffff;
        t3 += t2 >> 26; t2 &= 0x3ffffff; m = t2;
        t4 += t3 >> 26; t3 &= 0x3ffffff; m &= t3;
        t5 += t4 >> 26; t4 &= 0x3ffffff; m &= t4;
        t6 += t5 >> 26; t5 &= 0x3ffffff; m &= t5;
        t7 += t6 >> 26; t6 &= 0x3ffffff; m &= t6;
        t8 += t7 >> 26; t7 &= 0x3ffffff; m &= t7;
        t9 += t8 >> 26; t8 &= 0x3ffffff; m &= t8;

        debug_assert!(t9 >> 23 == 0);

        x = (t9 >> 22) | (if t9 == 0x03fffff { 1 } else { 0 } & if m == 0x3ffffff { 1 } else { 0 } & (if (t1 + 0x40 + ((t0 + 0x3d1) >> 26)) > 0x3ffffff { 1 } else { 0 }));

        t0 += x * 0x3d1; t1 += (x << 6);
        t1 += t0 >> 26; t0 &= 0x3ffffff;
        t2 += t1 >> 26; t1 &= 0x3ffffff;
        t3 += t2 >> 26; t2 &= 0x3ffffff;
        t4 += t3 >> 26; t3 &= 0x3ffffff;
        t5 += t4 >> 26; t4 &= 0x3ffffff;
        t6 += t5 >> 26; t5 &= 0x3ffffff;
        t7 += t6 >> 26; t6 &= 0x3ffffff;
        t8 += t7 >> 26; t7 &= 0x3ffffff;
        t9 += t8 >> 26; t8 &= 0x3ffffff;

        debug_assert!(t9 >> 22 == x);

        t9 &= 0x03fffff;

        self.n = [t0, t1, t2, t3, t4, t5, t6, t7, t8, t9];
    }

    /// Weakly normalize a field element: reduce it magnitude to 1,
    /// but don't fully normalize.
    pub fn normalize_weak(&mut self) {
        let mut t0 = self.n[0];
        let mut t1 = self.n[1];
        let mut t2 = self.n[2];
        let mut t3 = self.n[3];
        let mut t4 = self.n[4];
        let mut t5 = self.n[5];
        let mut t6 = self.n[6];
        let mut t7 = self.n[7];
        let mut t8 = self.n[8];
        let mut t9 = self.n[9];

        let x = t9 >> 22; t9 &= 0x03fffff;

        t0 += x * 0x3d1; t1 += x << 6;
        t1 += t0 >> 26; t0 &= 0x3ffffff;
        t2 += t1 >> 26; t1 &= 0x3ffffff;
        t3 += t2 >> 26; t2 &= 0x3ffffff;
        t4 += t3 >> 26; t3 &= 0x3ffffff;
        t5 += t4 >> 26; t4 &= 0x3ffffff;
        t6 += t5 >> 26; t5 &= 0x3ffffff;
        t7 += t6 >> 26; t6 &= 0x3ffffff;
        t8 += t7 >> 26; t7 &= 0x3ffffff;
        t9 += t8 >> 26; t8 &= 0x3ffffff;

        debug_assert!(t9 >> 23 == 0);

        self.n = [t0, t1, t2, t3, t4, t5, t6, t7, t8, t9];
    }

    /// Normalize a field element, without constant-time guarantee.
    pub fn normalize_var(&mut self) {
        let mut t0 = self.n[0];
        let mut t1 = self.n[1];
        let mut t2 = self.n[2];
        let mut t3 = self.n[3];
        let mut t4 = self.n[4];
        let mut t5 = self.n[5];
        let mut t6 = self.n[6];
        let mut t7 = self.n[7];
        let mut t8 = self.n[8];
        let mut t9 = self.n[9];

        let m: u32;
        let x = t9 >> 22; t9 &= 0x03fffff;

        t0 += x * 0x3d1; t1 += x << 6;
        t1 += t0 >> 26; t0 &= 0x3ffffff;
        t2 += t1 >> 26; t1 &= 0x3ffffff;
        t3 += t2 >> 26; t2 &= 0x3ffffff; m = t2;
        t4 += t3 >> 26; t3 &= 0x3ffffff; m &= t3;
        t5 += t4 >> 26; t4 &= 0x3ffffff; m &= t4;
        t6 += t5 >> 26; t5 &= 0x3ffffff; m &= t5;
        t7 += t6 >> 26; t6 &= 0x3ffffff; m &= t6;
        t8 += t7 >> 26; t7 &= 0x3ffffff; m &= t7;
        t9 += t8 >> 26; t8 &= 0x3ffffff; m &= t8;

        debug_assert!(t9 >> 23 == 0);

        x = (t9 >> 22) | (if t9 == 0x03fffff { 1 } else { 0 } & if m == 0x3ffffff { 1 } else { 0 } & (if (t1 + 0x40 + ((t0 + 0x3d1) >> 26)) > 0x3ffffff { 1 } else { 0 }));

        if (x > 0) {
            t0 += 0x3d1; t1 += x << 6;
            t1 += t0 >> 26; t0 &= 0x3ffffff;
            t2 += t1 >> 26; t1 &= 0x3ffffff;
            t3 += t2 >> 26; t2 &= 0x3ffffff;
            t4 += t3 >> 26; t3 &= 0x3ffffff;
            t5 += t4 >> 26; t4 &= 0x3ffffff;
            t6 += t5 >> 26; t5 &= 0x3ffffff;
            t7 += t6 >> 26; t6 &= 0x3ffffff;
            t8 += t7 >> 26; t7 &= 0x3ffffff;
            t9 += t8 >> 26; t8 &= 0x3ffffff;

            debug_assert!(t9 >> 22 == x);

            t9 &= 0x03fffff;
        }

        self.n = [t0, t1, t2, t3, t4, t5, t6, t7, t8, t9];
    }

    /// Verify whether a field element represents zero i.e. would
    /// normalize to a zero value. The field implementation may
    /// optionally normalize the input, but this should not be relied
    /// upon.
    pub fn normalizes_to_zero(&self) -> bool {
        let mut t0 = self.n[0];
        let mut t1 = self.n[1];
        let mut t2 = self.n[2];
        let mut t3 = self.n[3];
        let mut t4 = self.n[4];
        let mut t5 = self.n[5];
        let mut t6 = self.n[6];
        let mut t7 = self.n[7];
        let mut t8 = self.n[8];
        let mut t9 = self.n[9];

        let mut z0: u32; let mut z1: u32;

        let x = t9 >> 22; t9 &= 0x03fffff;

        t0 += x * 0x3d1; t1 += x << 6;
        t1 += t0 >> 26; t0 &= 0x3ffffff; z0  = t0; z1  = t0 ^ 0x3d0;
        t2 += t1 >> 26; t1 &= 0x3ffffff; z0 |= t1; z1 &= t1 ^ 0x40;
        t3 += t2 >> 26; t2 &= 0x3ffffff; z0 |= t2; z1 &= t2;
        t4 += t3 >> 26; t3 &= 0x3ffffff; z0 |= t3; z1 &= t3;
        t5 += t4 >> 26; t4 &= 0x3ffffff; z0 |= t4; z1 &= t4;
        t6 += t5 >> 26; t5 &= 0x3ffffff; z0 |= t5; z1 &= t5;
        t7 += t6 >> 26; t6 &= 0x3ffffff; z0 |= t6; z1 &= t6;
        t8 += t7 >> 26; t7 &= 0x3ffffff; z0 |= t7; z1 &= t7;
        t9 += t8 >> 26; t8 &= 0x3ffffff; z0 |= t8; z1 &= t8;
        z0 |= t9; z1 &= t9 ^ 0x3c00000;

        debug_assert!(t9 >> 23 == 0);

        return z0 == 0 || z1 == 0x3ffffff;
    }

    /// Verify whether a field element represents zero i.e. would
    /// normalize to a zero value. The field implementation may
    /// optionally normalize the input, but this should not be relied
    /// upon.
    pub fn normalizes_to_zero_var(&self) -> bool {
        let mut t0: u32; let mut t1: u32;
        let mut t2: u32; let mut t3: u32;
        let mut t4: u32; let mut t5: u32;
        let mut t6: u32; let mut t7: u32;
        let mut t8: u32; let mut t9: u32;
        let mut z0: u32; let mut z1: u32;
        let x: u32;

        t0 = self.n[0];
        t9 = self.n[9];

        x = t9 >> 22;
        t0 += x * 0x3d1;

        z0 = t0 & 0x3ffffff;
        z1 = z0 & 0x3d0;

        if z0 != 0 && z1 != 0x3ffffff {
            return false;
        }

        t1 = self.0[1];
        t2 = self.0[2];
        t3 = self.0[3];
        t4 = self.0[4];
        t5 = self.0[5];
        t6 = self.0[6];
        t7 = self.0[7];
        t8 = self.0[8];

        t9 &= 0x03fffff;
        t1 += x << 6;

        t1 += t0 >> 26;
        t2 += t1 >> 26; t1 &= 0x3ffffff; z0 |= t1; z1 &= t1 ^ 0x40;
        t3 += t2 >> 26; t2 &= 0x3ffffff; z0 |= t2; z1 &= t2;
        t4 += t3 >> 26; t3 &= 0x3ffffff; z0 |= t3; z1 &= t3;
        t5 += t4 >> 26; t4 &= 0x3ffffff; z0 |= t4; z1 &= t4;
        t6 += t5 >> 26; t5 &= 0x3ffffff; z0 |= t5; z1 &= t5;
        t7 += t6 >> 26; t6 &= 0x3ffffff; z0 |= t6; z1 &= t6;
        t8 += t7 >> 26; t7 &= 0x3ffffff; z0 |= t7; z1 &= t7;
        t9 += t8 >> 26; t8 &= 0x3ffffff; z0 |= t8; z1 &= t8;
        z0 |= t9; z1 &= t9 ^ 0x3c00000;

        debug_assert!(t9 >> 23 == 0);

        return z0 == 0 || z1 == 0x3ffffff;
    }

    /// Set a field element equal to a small integer. Resulting field
    /// element is normalized.
    pub fn set_int(&mut self, a: u32) {
        self.n = [a, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    }

    /// Verify whether a field element is zero. Requires the input to
    /// be normalized.
    pub fn is_zero(&self) -> bool {
        return (self.n[0] | self.n[1] | self.n[2] | self.n[3] | self.n[4] | self.n[5] | self.n[6] | self.n[7] | self.n[8] | self.n[9]) == 0;
    }

    /// Sets a field element equal to zero, initializing all fields.
    pub fn clear(&mut self) {
        self.n = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    }

    /// Set a field element equal to 32-byte big endian value. If
    /// successful, the resulting field element is normalized.
    pub fn set_b32(&mut self, a: [u8; 32]) -> bool {
        self.n[0] = (a[31] as u32) | ((a[30] as u32) << 8) | ((a[29] as u32) << 16) | (((a[28] & 0x3) as u32) << 24);
        self.n[1] = (((a[28] >> 2) & 0x3f) as u32) | ((a[27] as u32) << 6) | ((a[26] as u32) << 14) | (((a[25] & 0xf) as u32) << 22);
        self.n[2] = (((a[25] >> 4) & 0xf) as u32) | ((a[24] as u32) << 4) | ((a[23] as u32) << 12) | (((a[22] as u32) & 0x3f) << 20);
        self.n[3] = (((a[22] >> 6) & 0x3) as u32) | ((a[21] as u32) << 2) | ((a[20] as u32) << 10) | ((a[19] as u32) << 18);
        self.n[4] = (a[18] as u32) | ((a[17] as u32) << 8) | ((a[16] as u32) << 16) | (((a[15] & 0x3) as u32) << 24);
        self.n[5] = (((a[15] >> 2) & 0x3f) as u32) | ((a[14] as u32) << 6) | ((a[13] as u32) << 14) | (((a[12] as u32) & 0xf) << 22);
        self.n[6] = (((a[12] >> 4) & 0xf) as u32) | ((a[11] as u32) << 4) | ((a[10] as u32) << 12) | (((a[9] & 0x3f) as u32) << 20);
        self.n[7] = (((a[9] >> 6) & 0x3) as u32) | ((a[8] as u32) << 2) | ((a[7] as u32) << 10) | ((a[6] as u32) << 18);
        self.n[8] = (a[5] as u32) | ((a[4] as u32) << 8) | ((a[3] as u32) << 16) | (((a[2] & 0x3) as u32) << 24);
        self.n[9] = (((a[2] >> 2) & 0x3f) as u32) | ((a[1] as u32) << 6) | ((a[0] as u32) << 14);

        if self.n[9] == 0x03fffff && (self.n[8] & self.n[7] & self.n[6] & self.n[5] & self.n[4] & self.n[3] & self.n[2]) == 0x3ffffff && (self.n[1] + 0x40 + ((self.0 + 0x3d1) >> 26)) > 0x3ffffff {
            return false;
        }

        return true;
    }

    /// Convert a field element to a 32-byte big endian
    /// value. Requires the input to be normalized.
    pub fn b32(&self) -> [u8; 32] {

    }
}

impl Ord for Field {
    fn cmp(&self, other: &Field) -> Ordering {
        for i in (0..10).reverse() {
            if (self.n[i] > other.n[i]) {
                return Ordering::Greater;
            }
            if (self.n[i] < other.n[i]) {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Field) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct FieldStorage {
    n: [u32; 8],
}

impl FieldStorage {
    pub fn new(d7: u32, d6: u32, d5: u32, d4: u32, d3: u32, d2: u32, d1: u32, d0: u32) {
        FieldStorage {
            n: [d0, d1, d2, d3, d4, d5, d6, d7],
        },
    }
}
