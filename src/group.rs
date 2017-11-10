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
