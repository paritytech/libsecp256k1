use digest::generic_array::GenericArray;
use digest::Digest;
use crate::group::{Affine, Jacobian};
use crate::scalar::Scalar;
use crate::ecmult::ECMultContext;

impl ECMultContext {
    pub fn ecdh_raw<D: Digest + Default>(&self, point: &Affine, scalar: &Scalar) -> Option<GenericArray<u8, D::OutputSize>>
    {

        let mut digest: D = Default::default();

        let mut pt = point.clone();
        let s = scalar.clone();

        if s.is_zero() {
            return None;
        }

        let mut res = Jacobian::default();
        self.ecmult_const(&mut res, &pt, &s);
        pt.set_gej(&res);

        pt.x.normalize();
        pt.y.normalize();

        let x = pt.x.b32();
        let y = 0x02 | (if pt.y.is_odd() { 1 } else { 0 });
 
        digest.input(&[y]);
        digest.input(&x);
        Some(digest.result_reset())
    }
}
