use std::{env, io::Write, fs::File, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let const_path = Path::new(&out_dir).join("const.rs");
    let mut const_file = File::create(&const_path).expect("Create const.rs file failed");
    libsecp256k1_gen_ecmult::generate_to(&mut const_file).expect("Write const.rs file failed");
    const_file.flush().expect("Flush const.rs file failed");
}
