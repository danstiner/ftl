extern crate cmake;
use cmake::Config;

fn main() {
    let dst = Config::new("ftl-sdk")
        .define("DISABLE_FTL_APP", "ON")
        .define("FTL_STATIC_COMPILE", "ON")
        .define("FTL_CARGO_COMPILE", "ON")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=ftl");
}
