use std::{io::{Write, Error}, fs::File};
use libsecp256k1_core::curve::{Jacobian, Field, AffineStorage, Affine, AFFINE_G};
use libsecp256k1_core::util::{odd_multiples_table, ECMULT_TABLE_SIZE_G,
                              set_table_gej_var};

fn odd_multiples_table_storage_var(pre: &mut [AffineStorage],
                                   a: &Jacobian) {
    let mut prej: Vec<Jacobian> = Vec::with_capacity(pre.len());
    for _ in 0..pre.len() {
        prej.push(Jacobian::default());
    }
    let mut prea: Vec<Affine> = Vec::with_capacity(pre.len());
    for _ in 0..pre.len() {
        prea.push(Affine::default());
    }
    let mut zr: Vec<Field> = Vec::with_capacity(pre.len());
    for _ in 0..pre.len() {
        zr.push(Field::default());
    }

    odd_multiples_table(&mut prej, &mut zr, a);
    set_table_gej_var(&mut prea, &prej, &zr);

    for i in 0..pre.len() {
        pre[i] = prea[i].clone().into();
    }
}

pub fn generate_to(file: &mut File) -> Result<(), Error> {
    let mut gj = Jacobian::default();
    gj.set_ge(&AFFINE_G);
    let mut pre_g = Vec::with_capacity(ECMULT_TABLE_SIZE_G);
    for _ in 0..ECMULT_TABLE_SIZE_G {
        pre_g.push(AffineStorage::default());
    }
    odd_multiples_table_storage_var(&mut pre_g, &gj);
    file.write_fmt(format_args!("["))?;
    for pg in pre_g {
        file.write_fmt(
            format_args!(
                "    crate::curve::AffineStorage::new(crate::curve::FieldStorage::new({}, {}, {}, {}, {}, {}, {}, {}), crate::curve::FieldStorage::new({}, {}, {}, {}, {}, {}, {}, {})),",
                pg.x.0[7], pg.x.0[6], pg.x.0[5], pg.x.0[4], pg.x.0[3], pg.x.0[2], pg.x.0[1], pg.x.0[0],
                pg.y.0[7], pg.y.0[6], pg.y.0[5], pg.y.0[4], pg.y.0[3], pg.y.0[2], pg.y.0[1], pg.y.0[0]
            )
        )?;
    }
    file.write_fmt(format_args!("]"))?;

    Ok(())
}
