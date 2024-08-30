extern crate bindgen;

use std::env;

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=fusion.h");
    println!("cargo:rerun-if-changed=fusion.c");

    let bindings = bindgen::Builder::default()
        .header("fusion.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .include("Fusion/Fusion")
        .files(&[
            "Fusion/Fusion/FusionAhrs.c",
            "Fusion/Fusion/FusionCompass.c",
            "Fusion/Fusion/FusionOffset.c",
        ])
        .compile("fusion");

    let out_target_link = "-L".to_owned() + out_path.display().to_string().as_str();
    cc::Build::new()
        .file("fusion.c")
        .flag(&out_target_link) // link directory for original fusion library
        .flag("-lfusion")
        .compile("fusionrs");

    println!("cargo:rustc-link-search=native={}", out_path.display());
    println!("cargo:rustc-link-lib=static=fusion");
    println!("cargo:rustc-link-lib=static=fusionrs");
}
