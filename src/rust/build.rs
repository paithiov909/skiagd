use cxx_build::CFG;

fn main() {
  CFG.doxygen = true;
  cxx_build::bridge("src/lib.rs")
    .std("c++17")
    .compile("skiagd");

  println!("cargo:rerun-if-changed=src/lib.rs");
  println!("cargo:rerun-if-changed=/build.rs");
}
