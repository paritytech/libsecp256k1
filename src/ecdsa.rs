use field::Field;
use group::{Affine, Jacobian, AffineStorage};
use scalar::Scalar;
use ecmult::{ECMultContext, WINDOW_A, WINDOW_G};

const P_MINUS_ORDER: Field = field_const!(
    0, 0, 0, 1, 0x45512319, 0x50B75FC4, 0x402DA172, 0x2FC9BAEE
);

const ORDER_AS_FE: Field = field_const!(
    0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFE,
    0xBAAEDCE6, 0xAF48A03B, 0xBFD25E8C, 0xD0364141
);

impl ECMultContext {
    pub fn verify_raw(
        &self, sigr: &Scalar, sigs: &Scalar, pubkey: &Affine, message: &Scalar
    ) -> bool {
        let c;
        let (sn, u1, u2): (Scalar, Scalar, Scalar);

        if sigr.is_zero() || sigs.is_zero() {
            return false;
        }

        sn = sigs.inv_var();
        u1 = &sn * message;
        u2 = &sn * sigr;
        let mut pubkeyj: Jacobian = Jacobian::default();
        pubkeyj.set_ge(pubkey);
        let mut pr: Jacobian = Jacobian::default();
        self.ecmult(&mut pr, &pubkeyj, &u2, &u1);
        if pr.is_infinity() {
            return false;
        }

        c = sigr.b32();
        let mut xr: Field = Default::default();
        xr.set_b32(&c);

        if pr.eq_x_var(&xr) {
            return true;
        }
        if xr >= P_MINUS_ORDER {
            return false;
        }
        xr += ORDER_AS_FE;
        if pr.eq_x_var(&xr) {
            return true;
        }
        return false;
    }

    pub fn recover_raw(
        &self, sigr: &Scalar, sigs: &Scalar, rec_id: u8, message: &Scalar
    ) -> Option<Affine> {
        debug_assert!(rec_id >= 0 && rec_id < 4);

        if sigr.is_zero() || sigs.is_zero() {
            return None;
        }

        let brx = sigr.b32();
        let mut fx = Field::default();
        debug_assert!(fx.set_b32(&brx));

        if rec_id & 2 > 0 {
            if fx >= P_MINUS_ORDER {
                return None;
            }
            fx += ORDER_AS_FE;
        }
        let mut x = Affine::default();
        if !x.set_xo_var(&fx, rec_id & 1 > 0) {
            return None;
        }
        let mut xj = Jacobian::default();
        xj.set_ge(&x);
        let rn = sigr.inv();
        let mut u1 = &rn * message;
        u1 = u1.neg();
        let u2 = &rn * sigs;
        let mut qj = Jacobian::default();
        self.ecmult(&mut qj, &xj, &u2, &u1);

        let mut pubkey = Affine::default();
        pubkey.set_gej_var(&qj);

        if pubkey.is_infinity() {
            return None;
        } else {
            return Some(pubkey);
        }
    }
}
