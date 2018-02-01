use group::{Affine, Jacobian, AffineStorage, globalz_set_table_gej};
use field::Field;
use scalar::Scalar;

pub const WINDOW_A: usize = 5;
pub const WINDOW_G: usize = 16;
pub const ECMULT_TABLE_SIZE_A: usize = 1 << (WINDOW_A - 2);
pub const ECMULT_TABLE_SIZE_G: usize = 1 << (WINDOW_G - 2);

/// Context for accelerating the computation of a*P + b*G.
pub struct ECMultContext {
    pre_g: [AffineStorage; ECMULT_TABLE_SIZE_G],
}

/// Context for accelerating the computation of a*G.
pub struct ECMultGenContext {
    prec: [[AffineStorage; 16]; 64],
    blind: Scalar,
    initial: Jacobian,
}

/// A static ECMult context.
pub static ECMULT_CONTEXT: ECMultContext = ECMultContext {
    pre_g: include!("const.rs"),
};

/// A static ECMultGen context.
pub static ECMULT_GEN_CONTEXT: ECMultGenContext = ECMultGenContext {
    prec: include!("const_gen.rs"),
    blind: Scalar([2217680822, 850875797, 1046150361, 1330484644,
                   4015777837, 2466086288, 2052467175, 2084507480]),
    initial: Jacobian {
        x: field_const_raw!(586608, 43357028, 207667908, 262670128, 142222828, 38529388, 267186148, 45417712, 115291924, 13447464),
        y: field_const_raw!(12696548, 208302564, 112025180, 191752716, 143238548, 145482948, 228906000, 69755164, 243572800, 210897016),
        z: field_const_raw!(3685368, 75404844, 20246216, 5748944, 73206666, 107661790, 110806176, 73488774, 5707384, 104448710),
        infinity: false,
    }
};

pub fn odd_multiples_table(prej: &mut [Jacobian],
                       zr: &mut [Field],
                       a: &Jacobian) {
    debug_assert!(prej.len() == zr.len());
    debug_assert!(prej.len() > 0);
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
    for i in 1..prej.len() {
        prej[i] = prej[i-1].add_ge_var(&d_ge, Some(&mut zr[i]));
    }

    let l = &prej.last().unwrap().z * &d.z;
    prej.last_mut().unwrap().z = l;
}

fn odd_multiples_table_globalz_windowa(pre: &mut [Affine; ECMULT_TABLE_SIZE_A],
                                       globalz: &mut Field,
                                       a: &Jacobian) {
    let mut prej: [Jacobian; ECMULT_TABLE_SIZE_A] = Default::default();
    let mut zr: [Field; ECMULT_TABLE_SIZE_A] = Default::default();

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

fn table_get_ge_const(r: &mut Affine, pre: &[Affine], n: i32, w: usize) {
    let abs_n = n * (if n > 0 { 1 } else { 0 } * 2 - 1);
    let idx_n = abs_n / 2;
    debug_assert!(n & 1 == 1);
    debug_assert!(n >= -((1 << (w-1)) - 1));
    debug_assert!(n <=  ((1 << (w-1)) - 1));
    for m in 0..pre.len() {
        r.x.cmov(&pre[m].x, m == idx_n as usize);
        r.y.cmov(&pre[m].y, m == idx_n as usize);
    }
    r.infinity = false;
    let neg_y = r.y.neg(1);
    r.y.cmov(&neg_y, n != abs_n);
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

    debug_assert!(wanf.len() <= 256);
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
    pub fn ecmult(
        &self, r: &mut Jacobian, a: &Jacobian, na: &Scalar, ng: &Scalar
    ) {
        let mut tmpa = Affine::default();
        let mut pre_a: [Affine; ECMULT_TABLE_SIZE_A] = Default::default();
        let mut z = Field::default();
        let mut wnaf_na = [0i32; 256];
        let mut wnaf_ng = [0i32; 256];
        let bits_na = ecmult_wnaf(&mut wnaf_na, na, WINDOW_A);
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

impl ECMultGenContext {
    pub fn ecmult_gen(
        &self, r: &mut Jacobian, gn: &Scalar
    ) {
        let mut adds = AffineStorage::default();
        *r = self.initial.clone();

        let mut gnb = gn + &self.blind;
        let mut add = Affine::default();
        add.infinity = false;

        for j in 0..64 {
            let mut bits = gnb.bits(j * 4, 4);
            for i in 0..16 {
                adds.cmov(&self.prec[j][i], i as u32 == bits);
            }
            add = adds.clone().into();
            *r = r.add_ge(&add);
            #[allow(unused_assignments)]
            {
                bits = 0;
            }
        }
        add.clear();
        gnb.clear();
    }
}
