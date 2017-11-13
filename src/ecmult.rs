use group::{Affine, Jacobian, AffineStorage, set_table_gej_var, globalz_set_table_gej, AFFINE_G};
use field::Field;
use scalar::Scalar;

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

fn table_get_ge(r: &mut Affine, pre: &[Affine], n: i32, w: usize) {
    debug_assert!(n & 1 == 1);
    debug_assert!(n >= -((1 << (w-1)) - 1));
    debug_assert!(n <=  ((1 << (w-1)) - 1));
    if n > 0 {
        *r = pre[((n-1)/2) as usize].clone();
    } else {
        *r = pre[((-n-1)/2) as usize].neg();
    }
}

fn table_get_ge_storage(r: &mut Affine, pre: &[AffineStorage], n: i32, w: usize) {
    debug_assert!(n & 1 == 1);
    debug_assert!(n >= -((1 << (w-1)) - 1));
    debug_assert!(n <=  ((1 << (w-1)) - 1));
    if n > 0 {
        *r = pre[((n-1)/2) as usize].clone().into();
    } else {
        *r = pre[((-n-1)/2) as usize].clone().into();
        *r = r.neg();
    }
}

pub fn ecmult_wnaf(wanf: &mut [i32], a: &Scalar, w: usize) -> i32 {
    let mut s = a.clone();
    let mut last_set_bit: i32 = -1;
    let mut bit = 0;
    let mut sign = 1;
    let mut carry = 0;

    debug_assert!(wanf.len() >= 0 && wanf.len() <= 256);
    debug_assert!(w >= 2 && w <= 31);

    for i in 0..wanf.len() {
        wanf[i] = 0;
    }

    if s.bits(255, 1) > 0 {
        s = s.neg();
        sign = -1;
    }

    while bit < wanf.len() {
        let mut now;
        let mut word;
        if s.bits(bit, 1) == carry as u32 {
            bit += 1;
            continue;
        }

        now = w;
        if now > wanf.len() - bit {
            now = wanf.len() - bit;
        }

        word = (s.bits_var(bit, now) as i32) + carry;

        carry = (word >> (w-1)) & 1;
        word -= carry << w;

        wanf[bit] = sign * word;
        last_set_bit = bit as i32;

        bit += now;
    }
    debug_assert!(carry == 0);
    debug_assert!({
        let mut t = true;
        while bit < 256 {
            t = t && (s.bits(bit, 1) == 0);
            bit += 1;
        }
        t
    });
    last_set_bit + 1
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

    pub fn ecmult(
        &self, r: &mut Jacobian, a: &Jacobian, na: &Scalar, ng: Scalar
    ) {
        let mut tmpa = Affine::default();
        let mut pre_a: [Affine; ECMULT_TABLE_SIZE] = initialize_ecmult_table_size_array();
        let mut z = Field::default();
        let mut wnaf_na = [0i32; 256];
        let mut wnaf_ng = [0i32; 256];
        let mut bits_na = ecmult_wnaf(&mut wnaf_na, na, WINDOW_A);
        let mut bits = bits_na;
        odd_multiples_table_globalz_windowa(&mut pre_a, &mut z, a);

        let bits_ng = ecmult_wnaf(&mut wnaf_ng, &ng, WINDOW_G);
        if bits_ng > bits {
            bits = bits_ng;
        }

        r.set_infinity();
        for i in (0..bits).rev() {
            let mut n;
            *r = r.double_var(None);

            n = wnaf_na[i as usize];
            if i < bits_na && n != 0 {
                table_get_ge(&mut tmpa, &pre_a, n, WINDOW_A);
                *r = r.add_ge_var(&tmpa, None);
            }
            n = wnaf_ng[i as usize];
            if i < bits_ng && n != 0 {
                table_get_ge_storage(&mut tmpa, &self.pre_g, n, WINDOW_G);
                *r = r.add_zinv_var(&tmpa, &z);
            }
        }

        if !r.is_infinity() {
            r.z *= &z;
        }
    }
}
