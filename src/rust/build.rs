// use cxx_build::CFG;
// use std::path::Path;
// use std::{env, fs};

fn main() {
  // let cur_dir = env::current_dir().unwrap();
  // let out_dir = env::var("TARGET_DIR").unwrap_or_else(|_| "target".to_string());
  cxx_build::bridge("src/lib.rs")
    .std("c++17")
    .compile("skiagd");

  println!("cargo:rerun-if-changed=src/lib.rs");
  println!("cargo:rerun-if-changed=/build.rs");
}
