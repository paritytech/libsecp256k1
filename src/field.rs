pub struct Field {
    n: [u32; 10],
    magnitude: u32,
    normalized: bool,
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
            magnitude: 1,
            normalized: true,
        }
    }

    fn verify(&self) -> bool {
        let m = if self.normalized { 1 } else { 2 } * a.magnitude;
        let mut r = true;
        r &&= (self.n[0] <= 0x3ffffff * m);
        r &&= (self.n[1] <= 0x3ffffff * m);
        r &&= (self.n[2] <= 0x3ffffff * m);
        r &&= (self.n[3] <= 0x3ffffff * m);
        r &&= (self.n[4] <= 0x3ffffff * m);
        r &&= (self.n[5] <= 0x3ffffff * m);
        r &&= (self.n[6] <= 0x3ffffff * m);
        r &&= (self.n[7] <= 0x3ffffff * m);
        r &&= (self.n[8] <= 0x3ffffff * m);
        r &&= (self.n[9] <= 0x03fffff * m);
        r &&= (self.magnitude >= 0);
        r &&= (self.magnitude <= 32);
        if self.normalized {
            r &&= self.magnitude <= 1;
            if r && (self.n[9] == 0x03fffff) {
                let mid = self.n[8] & self.n[7] & self.n[6] & self.n[5] & self.n[4] & self.n[3] & self.n[2];
                if mid == 0x3ffffff {
                    r &&= ((self.n[1] + 0x40 + ((self.n[0] + 0x3d1) >> 26)) <= 0x3ffffff)
                }
            }
        }
        r
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
        self.magnitude = 1;
        self.normalized = true;
        debug_assert!(self.verify());
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
        self.magnitude = 1;
        debug_assert!(self.verify());
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
        self.magnitude = 1;
        self.normalized = true;
        debug_assert!(self.verify());
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
        self.magnitude = 1;
        self.normalized = true;
        debug_assert!(self.verify());
    }

    /// Verify whether a field element is zero. Requires the input to
    /// be normalized.
    pub fn is_zero(&self) -> bool {
        debug_assert!(self.normalized);
        debug_assert!(self.verify());
        return (self.n[0] | self.n[1] | self.n[2] | self.n[3] | self.n[4] | self.n[5] | self.n[6] | self.n[7] | self.n[8] | self.n[9]) == 0;
    }

    /// Check the "oddness" of a field element. Requires the input to
    /// be normalized.
    pub fn is_odd(&self) -> bool {
        debug_assert!(self.normalized);
        debug_assert!(self.verify());
        return self.n[0] & 1 != 0;
    }

    /// Sets a field element equal to zero, initializing all fields.
    pub fn clear(&mut self) {
        self.magnitude = 0;
        self.normalized = true;
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

        self.magnitude = 1;
        self.normalized = true;
        debug_assert!(self.verify());

        return true;
    }

    /// Convert a field element to a 32-byte big endian
    /// value. Requires the input to be normalized.
    pub fn b32(&self) -> [u8; 32] {
        debug_assert!(self.normalized);
        debug_assert!(self.verify());

        let mut r = [0u8; 32];

        r[0] = ((self.n[9] >> 14) & 0xff) as u8;
        r[1] = ((self.n[9] >> 6) & 0xff) as u8;
        r[2] = (((self.n[9] & 0x3f) << 2) | ((self.n[8] >> 24) & 0x3)) as u8;
        r[3] = ((self.n[8] >> 16) & 0xff) as u8;
        r[4] = ((self.n[8] >> 8) & 0xff) as u8;
        r[5] = (self.n[8] & 0xff) as u8;
        r[6] = ((self.n[7] >> 18) & 0xff) as u8;
        r[7] = ((self.n[7] >> 10) & 0xff) as u8;
        r[8] = ((self.n[7] >> 2) & 0xff) as u8;
        r[9] = (((self.n[7] & 0x3) << 6) | ((self.n[6] >> 20) & 0x3f)) as u8;
        r[10] = ((self.n[6] >> 12) & 0xff) as u8;
        r[11] = ((self.n[6] >> 4) & 0xff) as u8;
        r[12] = (((self.n[6] & 0xf) << 4) | ((self.n[5] >> 22) & 0xf)) as u8;
        r[13] = ((self.n[5] >> 14) & 0xff) as u8;
        r[14] = ((self.n[5] >> 6) & 0xff) as u8;
        r[15] = (((self.n[5] & 0x3f) << 2) | ((self.n[4] >> 24) & 0x3)) as u8;
        r[16] = ((self.n[4] >> 16) & 0xff) as u8;
        r[17] = ((self.n[4] >> 8) & 0xff) as u8;
        r[18] = (self.n[4] & 0xff) as u8;
        r[19] = ((self.n[3] >> 18) & 0xff) as u8;
        r[20] = ((self.n[3] >> 10) & 0xff) as u8;
        r[21] = ((self.n[3] >> 2) & 0xff) as u8;
        r[22] = (((self.n[3] & 0x3) << 6) | ((self.n[2] >> 20) & 0x3f)) as u8;
        r[23] = ((self.n[2] >> 12) & 0xff) as u8;
        r[24] = ((self.n[2] >> 4) & 0xff) as u8;
        r[25] = (((self.n[2] & 0xf) << 4) | ((self.n[1] >> 22) & 0xf)) as u8;
        r[26] = ((self.n[1] >> 14) & 0xff) as u8;
        r[27] = ((self.n[1] >> 6) & 0xff) as u8;
        r[28] = (((self.n[1] & 0x3f) << 2) | ((self.n[0] >> 24) & 0x3)) as u8;
        r[29] = ((self.n[0] >> 16) & 0xff) as u8;
        r[30] = ((self.n[0] >> 8) & 0xff) as u8;
        r[31] = (self.n[0] & 0xff) as u8;

        r
    }

    /// Set a field element equal to the additive inverse of
    /// another. Takes a maximum magnitude of the input as an
    /// argument. The magnitude of the output is one higher.
    pub fn negate(&mut self, other: &Field, m: u32) {
        debug_assert!(self.magnitude <= m);
        debug_assert!(self.verify());

        self.n[0] = 0x3fffc2f * 2 * (m + 1) - other.n[0];
        self.n[1] = 0x3ffffbf * 2 * (m + 1) - other.n[1];
        self.n[2] = 0x3ffffff * 2 * (m + 1) - other.n[2];
        self.n[3] = 0x3ffffff * 2 * (m + 1) - other.n[3];
        self.n[4] = 0x3ffffff * 2 * (m + 1) - other.n[4];
        self.n[5] = 0x3ffffff * 2 * (m + 1) - other.n[5];
        self.n[6] = 0x3ffffff * 2 * (m + 1) - other.n[6];
        self.n[7] = 0x3ffffff * 2 * (m + 1) - other.n[7];
        self.n[8] = 0x3ffffff * 2 * (m + 1) - other.n[8];
        self.n[9] = 0x03fffff * 2 * (m + 1) - other.n[9];

        self.magnitude = m + 1;
        self.normalized = false;
        debug_assert!(self.verify());
    }

    /// Multiplies the passed field element with a small integer
    /// constant. Multiplies the magnitude by that small integer.
    pub fn mul_int(&mut self, a: u32) {
        self.n[0] *= a;
        self.n[1] *= a;
        self.n[2] *= a;
        self.n[3] *= a;
        self.n[4] *= a;
        self.n[5] *= a;
        self.n[6] *= a;
        self.n[7] *= a;
        self.n[8] *= a;
        self.n[9] *= a;

        self.magnitude *= a;
        self.normalized = false;
        debug_assert!(self.verify());
    }

    /// Adds a field element to another. The result has the sum of the
    /// inputs' magnitudes as magnitude.
    pub fn add(&mut self, other: &Field) {
        self.n[0] += other.n[0];
        self.n[1] += other.n[1];
        self.n[2] += other.n[2];
        self.n[3] += other.n[3];
        self.n[4] += other.n[4];
        self.n[5] += other.n[5];
        self.n[6] += other.n[6];
        self.n[7] += other.n[7];
        self.n[8] += other.n[8];
        self.n[9] += other.n[9];

        self.magnitude += other.magnitude;
        self.normalized = false;
        debug_assert!(self.verify());
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Field) -> Ordering {
        // Variable time compare implementation.
        debug_assert!(self.normalized);
        debug_assert!(other.normalized);
        debug_assert!(self.verify());
        debug_assert!(other.verify());

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
