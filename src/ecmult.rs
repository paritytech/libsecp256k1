use group::{Affine, Jacobian, AffineStorage, set_table_gej_var, globalz_set_table_gej, AFFINE_G};
use field::Field;

pub const WINDOW_A: usize = 5;
pub const WINDOW_G: usize = 16;
pub const ECMULT_TABLE_SIZE: usize = 1 << (WINDOW_G - 2);

pub fn initialize_ecmult_table_size_array<T: Default>() -> [T; ECMULT_TABLE_SIZE] {
    use std::{mem, ptr};

    unsafe {
        let mut array: [T; ECMULT_TABLE_SIZE] = mem::uninitialized();

        for (i, element) in array.iter_mut().enumerate() {
            let foo = T::default();
            ptr::write(element, foo)
        }

        array
    }
}

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
    let mut prej: [Jacobian; ECMULT_TABLE_SIZE] = initialize_ecmult_table_size_array();
    let mut prea: [Affine; ECMULT_TABLE_SIZE] = initialize_ecmult_table_size_array();
    let mut zr: [Field; ECMULT_TABLE_SIZE] = initialize_ecmult_table_size_array();

    odd_multiples_table(&mut prej, &mut zr, a);
    set_table_gej_var(&mut prea, &prej, &zr);

    for i in 0..ECMULT_TABLE_SIZE {
        pre[i] = prea[i].clone().into();
    }
}

fn odd_multiples_table_globalz_windowa(pre: &mut [Affine; ECMULT_TABLE_SIZE],
                                       globalz: &mut Field,
                                       a: &Jacobian) {
    let mut prej: [Jacobian; ECMULT_TABLE_SIZE] = initialize_ecmult_table_size_array();
    let mut zr: [Field; ECMULT_TABLE_SIZE] = initialize_ecmult_table_size_array();

    odd_multiples_table(&mut prej, &mut zr, a);
    globalz_set_table_gej(pre, globalz, &prej, &zr);
}

impl ECMultContext {
    pub fn new() -> ECMultContext {
        let mut gj = Jacobian::default();
        gj.set_ge(&AFFINE_G);

        let mut ret = ECMultContext {
            pre_g: initialize_ecmult_table_size_array(),
        };

        /* precompute the tables with odd multiples */
        odd_multiples_table_storage_var(&mut ret.pre_g, &gj);

        ret
    }
}
