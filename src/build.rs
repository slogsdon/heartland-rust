extern crate serde_codegen;

use std::env;
use std::fs;
use std::path::Path;

pub fn main() {
    // a temporary directory where generated artifacts are stored
    let out_dir = env::var_os("OUT_DIR").unwrap();

    // the files containing the structs
    let credit_src = Path::new("src/entities/credit.in.rs");
    let portico_src = Path::new("src/entities/portico.in.rs");
    let soap_src = Path::new("src/entities/soap.in.rs");

    // a generated file that will contain the generated code
    let credit_dst = Path::new(&out_dir).join("entities/credit.rs");
    let portico_dst = Path::new(&out_dir).join("entities/portico.rs");
    let soap_dst = Path::new(&out_dir).join("entities/soap.rs");

    let _ = fs::create_dir_all(Path::new(&out_dir).join("entities"));

    serde_codegen::expand(&credit_src, &credit_dst).unwrap();
    serde_codegen::expand(&portico_src, &portico_dst).unwrap();
    serde_codegen::expand(&soap_src, &soap_dst).unwrap();
}
