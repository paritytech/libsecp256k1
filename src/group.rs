use field::{Field, FieldStorage};

macro_rules! affine_const {
    ($x: expr, $y: expr) => {
        $crate::group::Affine {
            x: $x, y: $y, infinity: false,
        }
    }
}

macro_rules! jacobian_const {
    ($x: expr, $y: expr) => {
        $crate::group::Jacobian {
            x: $x, y: $y, infinity: false,
            z: field_const!(0, 0, 0, 0, 0, 0, 0, 1),
        }
    }
}

macro_rules! affine_storage_const {
    ($x: expr, $y: expr) => {
        $crate::group::AffineStorage {
            x: $x, y: $y,
        }
    }
}

#[derive(Clone)]
pub struct Affine {
    pub(crate) x: Field,
    pub(crate) y: Field,
    pub(crate) infinity: bool,
}

#[derive(Clone)]
pub struct Jacobian {
    pub(crate) x: Field,
    pub(crate) y: Field,
    pub(crate) z: Field,
    pub(crate) infinity: bool,
}

#[derive(Clone)]
pub struct AffineStorage {
    pub(crate) x: Field,
    pub(crate) y: Field,
}

impl Default for Affine {
    fn default() -> Affine {
        Affine {
            x: Field::default(),
            y: Field::default(),
            infinity: false,
        }
    }
}

impl Default for Jacobian {
    fn default() -> Jacobian {
        Jacobian {
            x: Field::default(),
            y: Field::default(),
            z: Field::default(),
            infinity: false,
        }
    }
}

impl Default for AffineStorage {
    fn default() -> AffineStorage {
        AffineStorage {
            x: Field::default(),
            y: Field::default(),
        }
    }
}

pub(crate) static AFFINE_INFINITY: Affine = Affine {
    x: field_const!(0, 0, 0, 0, 0, 0, 0, 0),
    y: field_const!(0, 0, 0, 0, 0, 0, 0, 0),
    infinity: true,
};

pub(crate) static JACOBIAN_INFINITY: Jacobian = Jacobian {
    x: field_const!(0, 0, 0, 0, 0, 0, 0, 0),
    y: field_const!(0, 0, 0, 0, 0, 0, 0, 0),
    z: field_const!(0, 0, 0, 0, 0, 0, 0, 0),
    infinity: true,
};

pub(crate) static AFFINE_G: Affine = affine_const!(
    field_const!(
        0x79BE667E, 0xF9DCBBAC, 0x55A06295, 0xCE870B07,
        0x029BFCDB, 0x2DCE28D9, 0x59F2815B, 0x16F81798
    ),
    field_const!(
        0x483ADA77, 0x26A3C465, 0x5DA4FBFC, 0x0E1108A8,
        0xFD17B448, 0xA6855419, 0x9C47D08F, 0xFB10D4B8
    )
);

pub(crate) const CURVE_B: u32 = 7;

impl Affine {
    pub fn parse(p: &[u8; 65]) -> Option<Affine> {
        use {TAG_PUBKEY_HYBRID_EVEN, TAG_PUBKEY_HYBRID_ODD};

        if !(p[0] == 0x04 || p[0] == 0x06 || p[0] == 0x07) {
            return None;
        }
        let mut x = Field::default();
        let mut y = Field::default();
        let mut data = [0u8; 32];
        for i in 0..32 {
            data[i] = p[i+1];
        }
        if !x.set_b32(&data) {
            return None;
        }
        for i in 0..32 {
            data[i] = p[i+33];
        }
        if !y.set_b32(&data) {
            return None;
        }
        let mut elem = Affine::default();
        elem.set_xy(&x, &y);
        if (p[0] == TAG_PUBKEY_HYBRID_EVEN || p[0] == TAG_PUBKEY_HYBRID_ODD) &&
            (y.is_odd() != (p[0] == TAG_PUBKEY_HYBRID_ODD))
        {
            return None;
        }
        if elem.is_valid_var() {
            return Some(elem);
        } else {
            return None;
        }
    }

    /// Set a group element equal to the point with given X and Y
    /// coordinates.
    pub fn set_xy(&mut self, x: &Field, y: &Field) {
        self.infinity = false;
        self.x = x.clone();
        self.y = y.clone();
    }

    /// Set a group element (affine) equal to the point with the given
    /// X coordinate and a Y coordinate that is a quadratic residue
    /// modulo p. The return value is true iff a coordinate with the
    /// given X coordinate exists.
    pub fn set_xquad(&mut self, x: &Field) -> bool {
        self.x = x.clone();
        let x2 = x.sqr();
        let x3 = x * &x2;
        self.infinity = false;
        let mut c = Field::default();
        c.set_int(CURVE_B);
        c += &x3;
        let (v, ret) = c.sqrt();
        self.y = v;
        ret
    }

    /// Set a group element (affine) equal to the point with the given
    /// X coordinate, and given oddness for Y. Return value indicates
    /// whether the result is valid.
    pub fn set_xo_var(&mut self, x: &Field, odd: bool) -> bool {
        if !self.set_xquad(x) {
            return false;
        }
        self.y.normalize_var();
        if self.y.is_odd() != odd {
            self.y = self.y.neg(1);
        }
        return true;
    }

    /// Check whether a group element is the point at infinity.
    pub fn is_infinity(&self) -> bool {
        self.infinity
    }

    /// Check whether a group element is valid (i.e., on the curve).
    pub fn is_valid_var(&self) -> bool {
        if self.is_infinity() {
            return false;
        }
        let y2 = self.y.sqr();
        let mut x3 = self.x.sqr();
        x3 *= &self.x;
        let mut c = Field::default();
        c.set_int(CURVE_B);
        x3 += &c;
        x3.normalize_weak();
        y2.eq_var(&x3)
    }

    pub fn neg_in_place(&mut self, other: &Affine) {
        *self = other.clone();
        self.y.normalize_weak();
        self.y = self.y.neg(1);
    }

    pub fn neg(&self) -> Affine {
        let mut ret = Affine::default();
        ret.neg_in_place(self);
        ret
    }

    /// Set a group element equal to another which is given in
    /// jacobian coordinates.
    pub fn set_gej(&mut self, a: &Jacobian) {
        self.infinity = a.infinity;
        let mut a = a.clone();
        a.z.inv();
        let z2 = a.z.sqr();
        let z3 = &a.z * &z2;
        a.x *= &z2;
        a.y *= &z3;
        a.z.set_int(1);
        self.x = a.x;
        self.y = a.y;
    }

    pub fn set_gej_zinv(&mut self, a: &Jacobian, zi: &Field) {
        let zi2 = zi.sqr();
        let zi3 = &zi2 * &zi;
        self.x = &a.x * &zi2;
        self.y = &a.y * &zi3;
        self.infinity = a.infinity;
    }

    /// Clear a secp256k1_ge to prevent leaking sensitive information.
    pub fn clear(&mut self) {
        self.infinity = false;
        self.x.clear();
        self.y.clear();
    }
}

pub fn set_table_gej_var(r: &mut [Affine], a: &[Jacobian], zr: &[Field]) {
    debug_assert!(r.len() == a.len());

    let mut i = r.len() - 1;
    let mut zi: Field;

    if r.len() > 0 {
        zi = a[i].z.inv();
        r[i].set_gej_zinv(&a[i], &zi);

        while i > 0 {
            zi *= &zr[i];
            i -= 1;
            r[i].set_gej_zinv(&a[i], &zi);
        }
    }
}

pub fn globalz_set_table_gej(
    r: &mut [Affine], globalz: &mut Field, a: &[Jacobian], zr: &[Field]
) {
    debug_assert!(r.len() == a.len() && a.len() == zr.len());

    let mut i = r.len() - 1;
    let mut zs: Field;

    if r.len() > 0 {
        r[i].x = a[i].x.clone();
        r[i].y = a[i].y.clone();
        *globalz = a[i].z.clone();
        r[i].infinity = false;
        zs = zr[i].clone();

        while i > 0 {
            if i != r.len() - 1 {
                zs *= &zr[i];
            }
            i -= 1;
            r[i].set_gej_zinv(&a[i], &zs);
        }
    }
}

impl Jacobian {
    /// Set a group element (jacobian) equal to the point at infinity.
    pub fn set_infinity(&mut self) {
        self.infinity = true;
        self.x.clear();
        self.y.clear();
        self.z.clear();
    }

    /// Set a group element (jacobian) equal to another which is given
    /// in affine coordinates.
    pub fn set_ge(&mut self, a: &Affine) {
        self.infinity = a.infinity;
        self.x = a.x.clone();
        self.y = a.y.clone();
        self.z.set_int(1);
    }

    /// Compare the X coordinate of a group element (jacobian).
    pub fn eq_x_var(&self, x: &Field) -> bool {
        debug_assert!(!self.is_infinity());
        let mut r = self.z.sqr();
        r *= x;
        let mut r2 = self.x.clone();
        r2.normalize_weak();
        return r.eq_var(&r2);
    }

    /// Set r equal to the inverse of a (i.e., mirrored around the X
    /// axis).
    pub fn neg_in_place(&mut self, a: &Jacobian) {
        self.infinity = a.infinity;
        self.x = a.x.clone();
        self.y = a.y.clone();
        self.z = a.z.clone();
        self.y.normalize_weak();
        self.y = self.y.neg(1);
    }

    /// Check whether a group element is the point at infinity.
    pub fn is_infinity(&self) -> bool {
        self.infinity
    }

    /// Check whether a group element's y coordinate is a quadratic residue.
    pub fn has_quad_y_var(&self) -> bool {
        if self.infinity {
            return false;
        }

        let yz = &self.y * &self.z;
        return yz.is_quad_var();
    }

    /// Set r equal to the double of a. If rzr is not-NULL, r->z =
    /// a->z * *rzr (where infinity means an implicit z = 0). a may
    /// not be zero. Constant time.
    pub fn double_nonzero_in_place(&mut self, a: &Jacobian, rzr: Option<&mut Field>) {
        debug_assert!(!self.is_infinity());
        self.double_var_in_place(a, rzr);
    }

    /// Set r equal to the double of a. If rzr is not-NULL, r->z =
    /// a->z * *rzr (where infinity means an implicit z = 0).
    pub fn double_var_in_place(&mut self, a: &Jacobian, rzr: Option<&mut Field>) {
        self.infinity = a.infinity;
        if self.infinity {
            if let Some(rzr) = rzr {
                rzr.set_int(1);
            }
            return;
        }

        if let Some(rzr) = rzr {
            *rzr = a.y.clone();
            rzr.normalize_weak();
            rzr.mul_int(2);
        }

        self.z = &a.z * &a.y;
        self.z.mul_int(2);
        let mut t1 = a.x.sqr();
        t1.mul_int(3);
        let mut t2 = t1.sqr();
        let mut t3 = a.y.sqr();
        t3.mul_int(2);
        let mut t4 = t3.sqr();
        t4.mul_int(2);
        t3 *= &a.x;
        self.x = t3.clone();
        self.x.mul_int(4);
        self.x = self.x.neg(4);
        self.x += &t2;
        t2 = t2.neg(1);
        t3.mul_int(6);
        t3 += &t2;
        self.y = &t1 * &t3;
        t2 = t4.neg(2);
        self.y += &t2;
    }

    pub fn double_var(&self, rzr: Option<&mut Field>) -> Jacobian {
        let mut ret = Jacobian::default();
        ret.double_var_in_place(&self, rzr);
        ret
    }

    /// Set r equal to the sum of a and b. If rzr is non-NULL, r->z =
    /// a->z * *rzr (a cannot be infinity in that case).
    pub fn add_var_in_place(&mut self, a: &Jacobian, b: &Jacobian, rzr: Option<&mut Field>) {
        if a.is_infinity() {
            debug_assert!(rzr.is_none());
            *self = b.clone();
            return;
        }
        if b.is_infinity() {
            if let Some(rzr) = rzr {
                rzr.set_int(1);
            }
            *self = a.clone();
            return;
        }

        self.infinity = false;
        let mut z22 = b.z.sqr();
        let mut z12 = a.z.sqr();
        let mut u1 = &a.x * &z22;
        let mut u2 = &b.x * &z12;
        let mut s1 = &a.y * &z22; s1 *= &b.z;
        let mut s2 = &b.y * &z12; s2 *= &a.z;
        let mut h = u1.neg(1); h += &u2;
        let mut i = s1.neg(1); i += &s2;
        if h.normalizes_to_zero_var() {
            if i.normalizes_to_zero_var() {
                self.double_var_in_place(a, rzr);
            } else {
                if let Some(rzr) = rzr {
                    rzr.set_int(0);
                }
                self.infinity = true;
            }
            return;
        }
        let mut i2 = i.sqr();
        let mut h2 = h.sqr();
        let mut h3 = &h * &h2;
        h *= &b.z;
        if let Some(rzr) = rzr {
            *rzr = h.clone();
        }
        self.z = &a.z * &h;
        let mut t = &u1 * &h2;
        self.x = t.clone(); self.x.mul_int(2); self.x += &h3;
        self.x = self.x.neg(3); self.x += &i2;
        self.y = self.x.neg(5); self.y += &t; self.y *= &i;
        h3 *= &s1; h3 = h3.neg(1);
        self.y += &h3;
    }

    /// Set r equal to the sum of a and b (with b given in affine
    /// coordinates, and not infinity).
    pub fn add_ge_in_place(&mut self, a: &Jacobian, b: &Affine) {
        const FE1: Field = field_const!(0, 0, 0, 0, 0, 0, 0, 0);

        debug_assert!(!b.infinity);

        let mut zz = a.z.sqr();
        let mut u1 = a.x.clone(); u1.normalize_weak();
        let mut u2 = &b.x * &zz;
        let mut s1 = a.y.clone(); s1.normalize_weak();
        let mut s2 = &b.y * &zz;
        s2 *= &a.z;
        let mut t = u1.clone(); t += &u2;
        let mut m = s1.clone(); m += &s2;
        let mut rr = t.sqr();
        let mut m_alt = u2.neg(1);
        let mut tt = &u1 * &m_alt;
        rr += &tt;
        let mut degenerate = m.normalizes_to_zero() && rr.normalizes_to_zero();
        let mut rr_alt = s1.clone();
        rr_alt.mul_int(2);
        m_alt += &u1;

        rr_alt.cmov(&rr, !degenerate);
        m_alt.cmov(&m, !degenerate);

        let mut n = m_alt.sqr();
        let mut q = &n * &t;

        n = n.sqr();
        n.cmov(&m, degenerate);
        t = rr_alt.sqr();
        self.z = &a.z * &m_alt;
        let mut infinity = {
            let p = self.z.normalizes_to_zero();
            let q = a.infinity;

            match (p, q) {
                (true, true) => false,
                (true, false) => true,
                (false, true) => false,
                (false, false) => false,
            }
        };
        self.z.mul_int(2);
        q = q.neg(1);
        t += &q;
        t.normalize_weak();
        self.x = t.clone();
        t.mul_int(2);
        t += &q;
        t *= &rr_alt;
        t += &n;
        self.y = t.neg(3);
        self.y.normalize_weak();
        self.x.mul_int(4);
        self.y.mul_int(4);

        self.x.cmov(&b.x, a.infinity);
        self.y.cmov(&b.y, a.infinity);
        self.z.cmov(&FE1, a.infinity);
        self.infinity = infinity;
    }

    /// Set r equal to the sum of a and b (with b given in affine
    /// coordinates). This is more efficient than
    /// secp256k1_gej_add_var. It is identical to secp256k1_gej_add_ge
    /// but without constant-time guarantee, and b is allowed to be
    /// infinity. If rzr is non-NULL, r->z = a->z * *rzr (a cannot be
    /// infinity in that case).
    pub fn add_ge_var_in_place(&mut self, a: &Jacobian, b: &Affine, rzr: Option<&mut Field>) {
        if a.is_infinity() {
            debug_assert!(rzr.is_none());
            self.set_ge(b);
            return;
        }
        if b.is_infinity() {
            if let Some(rzr) = rzr {
                rzr.set_int(1);
            }
            *self = a.clone();
            return;
        }
        self.infinity = false;

        let mut z12 = a.z.sqr();
        let mut u1 = a.x.clone(); u1.normalize_weak();
        let mut u2 = &b.x * &z12;
        let mut s1 = a.y.clone(); s1.normalize_weak();
        let mut s2 = &b.y * &z12; s2 *= &a.z;
        let mut h = u1.neg(1); h += &u2;
        let mut i = s1.neg(1); i += &s2;
        if h.normalizes_to_zero_var() {
            if i.normalizes_to_zero_var() {
                self.double_var_in_place(a, rzr);
            } else {
                if let Some(rzr) = rzr {
                    rzr.set_int(0);
                }
                self.infinity = true;
            }
            return;
        }
        let mut i2 = i.sqr();
        let mut h2 = h.sqr();
        let mut h3 = &h * &h2;
        if let Some(rzr) = rzr {
            *rzr = h.clone();
        }
        self.z = &a.z * &h;
        let mut t = &u1 * &h2;
        self.x = t.clone(); self.x.mul_int(2); self.x += &h3;
        self.x = self.x.neg(3); self.x += &i2;
        self.y = self.x.neg(5); self.y += &t; self.y *= &i;
        h3 *= &s1; h3 = h3.neg(1);
        self.y += &h3;
    }

    pub fn add_ge_var(&self, b: &Affine, rzr: Option<&mut Field>) -> Jacobian {
        let mut ret = Jacobian::default();
        ret.add_ge_var_in_place(&self, b, rzr);
        ret
    }

    /// Set r equal to the sum of a and b (with the inverse of b's Z
    /// coordinate passed as bzinv).
    pub fn add_zinv_var_in_place(&mut self, a: &Jacobian, b: &Affine, bzinv: &Field) {
        if b.is_infinity() {
            *self = a.clone();
            return;
        }
        if a.is_infinity() {
            self.infinity = b.infinity;
            let mut bzinv2 = bzinv.sqr();
            let mut bzinv3 = &bzinv2 * bzinv;
            self.x = &b.x * &bzinv2;
            self.y = &b.y * &bzinv3;
            self.z.set_int(1);
            return;
        }
        self.infinity = false;

        let mut az = &a.z * &bzinv;
        let mut z12 = az.sqr();
        let mut u1 = a.x.clone(); u1.normalize_weak();
        let mut u2 = &b.x * &z12;
        let mut s1 = a.y.clone(); s1.normalize_weak();
        let mut s2 = &b.y * &z12; s2 *= &az;
        let mut h = u1.neg(1); h += &u2;
        let mut i = s1.neg(1); i += &s2;
        if h.normalizes_to_zero_var() {
            if i.normalizes_to_zero_var() {
                self.double_var_in_place(a, None);
            } else {
                self.infinity = true;
            }
            return;
        }
        let mut i2 = i.sqr();
        let mut h2 = h.sqr();
        let mut h3 = &h * &h2;
        self.z = a.z.clone(); self.z *= &h;
        let mut t = &u1 * &h2;
        self.x = t.clone(); self.x.mul_int(2); self.x += &h3;
        self.x = self.x.neg(3); self.x += &i2;
        self.y = self.x.neg(5); self.y += &t; self.y *= &i;
        h3 *= &s1; h3 = h3.neg(1);
        self.y += &h3;
    }

    pub fn add_zinv_var(&mut self, b: &Affine, bzinv: &Field) -> Jacobian {
        let mut ret = Jacobian::default();
        ret.add_zinv_var_in_place(&self, b, bzinv);
        ret
    }

    /// Clear a secp256k1_gej to prevent leaking sensitive
    /// information.
    pub fn clear(&mut self) {
        self.infinity = false;
        self.x.clear();
        self.y.clear();
        self.z.clear();
    }

    /// Rescale a jacobian point by b which must be
    /// non-zero. Constant-time.
    pub fn rescale(&mut self, s: &Field) {
        debug_assert!(!s.is_zero());
        let mut zz = s.sqr();
        self.x *= &zz;
        self.y *= &zz;
        self.y *= s;
        self.z *= s;
    }
}

impl From<AffineStorage> for Affine {
    fn from(a: AffineStorage) -> Affine {
        affine_const!(
            a.x.into(),
            a.y.into()
        )
    }
}

impl Into<AffineStorage> for Affine {
    fn into(mut self) -> AffineStorage {
        debug_assert!(!self.is_infinity());
        self.x.normalize();
        self.y.normalize();
        affine_storage_const!(
            self.x.into(),
            self.y.into()
        )
    }
}

impl AffineStorage {
    /// If flag is true, set *r equal to *a; otherwise leave
    /// it. Constant-time.
    pub fn cmov(&mut self, a: &AffineStorage, flag: bool) {
        self.x.cmov(&a.x, flag);
        self.y.cmov(&a.y, flag);
    }
}
