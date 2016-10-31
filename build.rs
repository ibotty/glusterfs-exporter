extern crate serde_codegen;

use std::env;
use std::fs::create_dir_all;
use std::path::Path;

pub fn main() {
    // a temporary directory where generated artifacts are stored
    let out_dir = env::var_os("OUT_DIR").unwrap();
    // the file containing the structs
    let src = Path::new("src/glusterfs_exporter/types.in.rs");
    // a generated file that will contain the generated code
    let dstdir = Path::new(&out_dir).join("glusterfs_exporter");
    let dst = dstdir.join("types.rs");

    create_dir_all(&dstdir).unwrap();

    serde_codegen::expand(&src, &dst).unwrap();
}
