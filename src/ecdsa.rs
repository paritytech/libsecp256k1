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
        let mut c = [0u8; 32];
        let (mut sn, mut u1, mut u2): (Scalar, Scalar, Scalar);
        let pubkeyj: Jacobian;
        let pr: Jacobian;

        if sigr.is_zero() || sigs.is_zero() {
            return false;
        }

        sn = sigs.inv_var();
        u1 = &sn * message;
        u2 = &sn * sigr;
        pubkeyj.set_ge(pubkey);
        self.ecmult(&pr, &pubkeyj, &u2, &u1);
        if pr.is_infinity() {
            return false;
        }

        let c = sigr.b32();
        xr.set_b32(c);

        if xr.eq_x_var(pr) {
            return true;
        }
        if xr >= P_MINUS_ORDER {
            return false;
        }
        xr += ORDER_AS_FE;
        if xr.eq_x_var(pr) {
            return true;
        }
        return false;
    }
}
