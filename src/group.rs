use field::Field;

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

pub struct Affine {
    pub(crate) x: Field,
    pub(crate) y: Field,
    pub(crate) infinity: bool,
}

pub struct Jacobian {
    pub(crate) x: Field,
    pub(crate) y: Field,
    pub(crate) z: Field,
    pub(crate) infinity: bool,
}

pub struct AffineStorage {
    pub(crate) x: Field,
    pub(crate) y: Field,
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
    /// Set a group element equal to the point with given X and Y
    /// coordinates.
    pub fn set_xy(&mut self, x: &Field, y: &Field) {

    }

    /// Set a group element (affine) equal to the point with the given
    /// X coordinate and a Y coordinate that is a quadratic residue
    /// modulo p. The return value is true iff a coordinate with the
    /// given X coordinate exists.
    pub fn set_xquad(&mut self, x: &Field) -> bool {

    }

    /// Set a group element (affine) equal to the point with the given
    /// X coordinate, and given oddness for Y. Return value indicates
    /// whether the result is valid.
    pub fn set_xo_var(&mut self, x: &Field, odd: bool) -> bool {

    }

    /// Check whether a group element is the point at infinity.
    pub fn is_infinity(&self) -> bool {

    }

    /// Check whether a group element is valid (i.e., on the curve).
    pub fn is_valid_var(&self) -> bool {

    }

    pub fn neg_in_place(&self, other: &Affine) {

    }

    /// Set a group element equal to another which is given in
    /// jacobian coordinates.
    pub fn set_gej(&mut self, a: &Jacobian) {

    }

    /// Clear a secp256k1_ge to prevent leaking sensitive information.
    pub fn clear(&mut self) {

    }
}

impl Jacobian {
    /// Set a group element (jacobian) equal to the point at infinity.
    pub fn set_infinity(&mut self) {

    }

    /// Set a group element (jacobian) equal to another which is given
    /// in affine coordinates.
    pub fn set_ge(&mut self, a: &Affine) {

    }

    /// Compare the X coordinate of a group element (jacobian).
    pub fn eq_x_var(&self, a: &Jacobian) -> bool {

    }

    /// Set r equal to the inverse of a (i.e., mirrored around the X
    /// axis).
    pub fn neg_in_place(&mut self, a: &Jacobian) {

    }

    /// Check whether a group element is the point at infinity.
    pub fn is_infinity(&self) -> bool {

    }

    /// Check whether a group element's y coordinate is a quadratic residue.
    pub fn has_quad_y_var(&self) -> bool {

    }

    /// Set r equal to the double of a. If rzr is not-NULL, r->z =
    /// a->z * *rzr (where infinity means an implicit z = 0). a may
    /// not be zero. Constant time.
    pub fn double_nonzero_in_place(&mut self, a: &Jacobian, rzr: Option<&Field>) {

    }

    /// Set r equal to the double of a. If rzr is not-NULL, r->z =
    /// a->z * *rzr (where infinity means an implicit z = 0).
    pub fn double_var_in_place(&mut self, a: &Jacobian, rzr: Option<&Field>) {

    }

    /// Set r equal to the sum of a and b. If rzr is non-NULL, r->z =
    /// a->z * *rzr (a cannot be infinity in that case).
    pub fn add_var_in_place(&mut self, a: &Jacobian, b: &Jacobian, rzr: Option<&Field>) {

    }

    /// Set r equal to the sum of a and b (with b given in affine
    /// coordinates, and not infinity).
    pub fn add_ge_in_place(&mut self, a: &Jacobian, b: &Affine) {

    }

    /// Set r equal to the sum of a and b (with b given in affine
    /// coordinates). This is more efficient than
    /// secp256k1_gej_add_var. It is identical to secp256k1_gej_add_ge
    /// but without constant-time guarantee, and b is allowed to be
    /// infinity. If rzr is non-NULL, r->z = a->z * *rzr (a cannot be
    /// infinity in that case).
    pub fn add_ge_var_in_place(&mut self, a: &Jacobian, b: &Affine, rzr: Option<&Field>) {

    }

    /// Set r equal to the sum of a and b (with the inverse of b's Z
    /// coordinate passed as bzinv).
    pub fn add_zinv_var_in_place(&mut self, a: Jacobian, b: &Affine, bzinv: &Field) {

    }

    /// Clear a secp256k1_gej to prevent leaking sensitive
    /// information.
    pub fn clear(&mut self) {

    }

    /// Rescale a jacobian point by b which must be
    /// non-zero. Constant-time.
    pub fn rescale(&mut self, b: &Field) {

    }
}

impl From<AffineStorage> for Affine {
    fn from(a: AffineStorage) -> Affine {

    }
}

impl Into<AffineStorage> for Affine {
    fn into(self) -> AffineStorage {

    }
}

impl AffineStorage {
    /// If flag is true, set *r equal to *a; otherwise leave
    /// it. Constant-time.
    pub fn cmov(&mut self, a: &AffineStorage, flag: bool) {

    }
}
