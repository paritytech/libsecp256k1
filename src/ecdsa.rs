const WINDOW_A: usize = 5;
const WINDOW_G: usize = 5;

const P_MINUS_ORDER: Field = field_const!(
    0, 0, 0, 1, 0x45512319, 0x50B75FC4, 0x402DA172, 0x2FC9BAEE
);

const ORDER_AS_FE: Field = field_const!(
    0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFE,
    0xBAAEDCE6, 0xAF48A03B, 0xBFD25E8C, 0xD0364141
);

fn table_get_ge(r, pre, n, w) {
    debug_assert!(n & 1 == 1);
    debug_assert!(n >= -((1 << (w-1)) - 1));
    debug_assert!(n <=  ((1 << (w-1)) - 1));
    if n > 0 {
        *r = pre[(n-1)/2];
    } else {
        *r = pre[(-n-1)/2].neg();
    }
}

fn table_get_ge_storage(r, pre, n, w) {
    debug_assert!(n & 1 == 1);
    debug_assert!(n >= -((1 << (w-1)) - 1));
    debug_assert!(n <=  ((1 << (w-1)) - 1));
    if n > 0 {
        *r = pre[(n-1)/2].into();
    } else {
        *r = pre[(-n-1)/2].into();
        *r = r.neg();
    }
}

pub fn ecmult_wnaf(wanf: &mut [i32], a: &Scalar, w: usize) -> i32 {
    let mut s = a.clone();
    let mut last_set_bit = -1;
    let mut bit = 0;
    let mut sign = 1;
    let mut carry = 0;

    debug_assert!(wanf.len() >= 0 && wanf.len() <= 256);
    debug_assert!(w >= 2 && w <= 31);

    for v in wanf {
        *v = 0;
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
        last_set_bit = bit;

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
        &self, r: &mut Jacobian, a: &Jacobian, na: &Scalar, ng: Scalar
    ) {
        let mut tempa = Affine::default();
        let mut pre_a: [Affine; ECMULT_TABLE_SIZE] = Default::default();
        let mut z = Field::default();
        let mut wnaf_na = [0i32; 256];
        let mut wanf_ng = [0i32; 256];
        let mut bits_na = ecmult_wnaf(&mut wnaf_na, na, WINDOW_A);
        let mut bits = bits_na;
        odd_multiples_table_globalz_windowa(&mut pre_a, &mut z, a);

        let bits_ng = ecmult_wanf(&mut wanf_ng, ng, WINDOW_G);
        if bits_ng > bits {
            bits = bits_ng;
        }

        r.set_infinity();
        for i in (0..bits).rev() {
            let n;
            r = r.double_var(None);

            n = wnaf_na[i];
            if i < bits_na && n != 0 {
                table_get_ge(&mut tmpa, &prea, WINDOW_A);
                r = r.add_ge_var(&tmpa, None);
            }
            n = wnaf_ng[i];
            if i < bits_ng && n != 0 {
                table_get_ge_storage(&mut tmpa, &ctx.pre_g, WINDOW_G);
                r = r.add_zinv_var(&tmpa, &z);
            }
        }

        if !r.is_infinity() {
            r.z *= &z;
        }
    }

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
