use std::{io::{Write, Error}, fs::File};
use libsecp256k1_core::curve::{Jacobian, Field, AffineStorage, Affine, AFFINE_G};

pub fn set_all_gej_var(a: &[Jacobian]) -> Vec<Affine> {
    let mut az: Vec<Field> = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        if !a[i].is_infinity() {
            az.push(a[i].z.clone());
        }
    }
    let azi: Vec<Field> = inv_all_var(&az);

    let mut ret = Vec::with_capacity(a.len());
    for _ in 0..a.len() {
        ret.push(Affine::default());
    }

    let mut count = 0;
    for i in 0..a.len() {
        ret[i].infinity = a[i].infinity;
        if !a[i].is_infinity() {
            ret[i].set_gej_zinv(&a[i], &azi[count]);
            count += 1;
        }
    }
    ret
}

/// Calculate the (modular) inverses of a batch of field
/// elements. Requires the inputs' magnitudes to be at most 8. The
/// output magnitudes are 1 (but not guaranteed to be
/// normalized). The inputs and outputs must not overlap in
/// memory.
pub fn inv_all_var(fields: &[Field]) -> Vec<Field> {
    if fields.len() == 0 {
        return Vec::new();
    }

    let mut ret = Vec::new();
    ret.push(fields[0].clone());

    for i in 1..fields.len() {
        ret.push(Field::default());
        ret[i] = &ret[i - 1] * &fields[i];
    }

    let mut u = ret[fields.len() - 1].inv_var();

    for i in (1..fields.len()).rev() {
        let j = i;
        let i = i - 1;
        ret[j] = &ret[i] * &u;
        u = &u * &fields[j];
    }

    ret[0] = u;
    ret
}

pub fn generate_to(file: &mut File) -> Result<(), Error> {
    let mut gj = Jacobian::default();
    gj.set_ge(&AFFINE_G);

    // Construct a group element with no known corresponding scalar (nothing up my sleeve).
    let mut nums_32 = [0u8; 32];
    debug_assert!("The scalar for this x is unknown".as_bytes().len() == 32);
    for (i, v) in "The scalar for this x is unknown".as_bytes().iter().enumerate() {
        nums_32[i] = *v;
    }
    let mut nums_x = Field::default();
    debug_assert!(nums_x.set_b32(&nums_32));
    let mut nums_ge = Affine::default();
    debug_assert!(nums_ge.set_xo_var(&nums_x, false));
    let mut nums_gej = Jacobian::default();
    nums_gej.set_ge(&nums_ge);
    nums_gej = nums_gej.add_ge_var(&AFFINE_G, None);

    // Compute prec.
    let mut precj: Vec<Jacobian> = Vec::with_capacity(1024);
    for _ in 0..1024 {
        precj.push(Jacobian::default());
    }
    let mut gbase = gj.clone();
    let mut numsbase = nums_gej.clone();
    for j in 0..64 {
        precj[j*16] = numsbase.clone();
        for i in 1..16 {
            precj[j*16 + i] = precj[j*16 + i - 1].add_var(&gbase, None);
        }
        for _ in 0..4 {
            gbase = gbase.double_var(None);
        }
        numsbase = numsbase.double_var(None);
        if j == 62 {
            numsbase = numsbase.neg();
            numsbase = numsbase.add_var(&nums_gej, None);
        }
    }
    let prec = set_all_gej_var(&precj);
    file.write_fmt(format_args!("["))?;
    for j in 0..64 {
        file.write_fmt(format_args!("    ["))?;
        for i in 0..16 {
            let pg: AffineStorage = prec[j*16 + i].clone().into();
            file.write_fmt(format_args!(
                "        crate::curve::AffineStorage::new(crate::curve::FieldStorage::new({}, {}, {}, {}, {}, {}, {}, {}), crate::curve::FieldStorage::new({}, {}, {}, {}, {}, {}, {}, {})),",
                pg.x.0[7], pg.x.0[6], pg.x.0[5], pg.x.0[4], pg.x.0[3], pg.x.0[2], pg.x.0[1], pg.x.0[0],
                pg.y.0[7], pg.y.0[6], pg.y.0[5], pg.y.0[4], pg.y.0[3], pg.y.0[2], pg.y.0[1], pg.y.0[0]
            ))?;
        }
        file.write_fmt(format_args!("    ],"))?;
    }
    file.write_fmt(format_args!("]"))?;

    Ok(())
}
