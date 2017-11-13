use group::{Affine, Jacobian, AffineStorage, set_table_gej_var, AFFINE_G};
use field::Field;

const WINDOW_G: usize = 5;
const ECMULT_TABLE_SIZE: usize = 1 << (WINDOW_G - 2);

pub struct ECMultContext {
    pre_g: [AffineStorage; ECMULT_TABLE_SIZE],
}

fn odd_multiples_table(prej: &mut [Jacobian; ECMULT_TABLE_SIZE],
                       zr: &mut [Field; ECMULT_TABLE_SIZE],
                       a: &Jacobian) {
    debug_assert!(!a.is_infinity());

    let d = a.double_var(None);
    let d_ge = Affine {
        x: d.x.clone(),
        y: d.y.clone(),
        infinity: false,
    };

    let mut a_ge = Affine::default();
    a_ge.set_gej_zinv(a, &d.z);
    prej[0].x = a_ge.x;
    prej[0].y = a_ge.y;
    prej[0].z = a.z.clone();
    prej[0].infinity = false;

    zr[0] = d.z.clone();
    for i in 1..ECMULT_TABLE_SIZE {
        prej[i] = prej[i-1].add_ge_var(&d_ge, Some(&mut zr[i]));
    }

    let l = &prej.last().unwrap().z * &d.z;
    prej.last_mut().unwrap().z = l;
}

fn odd_multiples_table_storage_var(pre: &mut [AffineStorage; ECMULT_TABLE_SIZE],
                                   a: &Jacobian) {
    let mut prej: [Jacobian; ECMULT_TABLE_SIZE] = Default::default();
    let mut prea: [Affine; ECMULT_TABLE_SIZE] = Default::default();
    let mut zr: [Field; ECMULT_TABLE_SIZE] = Default::default();

    odd_multiples_table(&mut prej, &mut zr, a);
    set_table_gej_var(&mut prea, &prej, &zr);

    for i in 0..ECMULT_TABLE_SIZE {
        pre[i] = prea[i].clone().into();
    }
}

impl ECMultContext {
    pub fn new() -> ECMultContext {
        let mut gj = Jacobian::default();
        gj.set_ge(&AFFINE_G);

        let mut ret = ECMultContext {
            pre_g: Default::default(),
        };

        /* precompute the tables with odd multiples */
        odd_multiples_table_storage_var(&mut ret.pre_g, &gj);

        ret
    }
}
