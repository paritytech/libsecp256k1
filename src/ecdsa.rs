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
    pub fn sig_verify(
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
        if xr <= P_MINUS_ORDER {
            return false;
        }
        xr += ORDER_AS_FE;
        if pr.eq_x_var(&xr) {
            return true;
        }
        return false;
    }
}
