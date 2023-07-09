use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("./vendor/src")
        .canonicalize()
        .expect("Unable to find libdir");

    let headers_path = libdir_path.join("rebound.h");
    let headers_path_str = headers_path
        .to_str()
        .expect("Path to headers is not valid UTF-8");

    println!("cargo:rerun-if-changed={}", headers_path_str);
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib=rebound");

    cc::Build::new()
        .files(vec![
            "vendor/src/rebound.c",
            "vendor/src/tree.c",
            "vendor/src/particle.c",
            "vendor/src/gravity.c",
            "vendor/src/integrator.c",
            "vendor/src/integrator_whfast.c",
            "vendor/src/integrator_saba.c",
            "vendor/src/integrator_ias15.c",
            "vendor/src/integrator_sei.c",
            "vendor/src/integrator_bs.c",
            "vendor/src/integrator_leapfrog.c",
            "vendor/src/integrator_mercurius.c",
            "vendor/src/integrator_eos.c",
            "vendor/src/integrator_tes.c",
            "vendor/src/boundary.c",
            "vendor/src/input.c",
            "vendor/src/binarydiff.c",
            "vendor/src/output.c",
            "vendor/src/collision.c",
            "vendor/src/communication_mpi.c",
            "vendor/src/display.c",
            "vendor/src/tools.c",
            "vendor/src/rotations.c",
            "vendor/src/derivatives.c",
            "vendor/src/simulationarchive.c",
            "vendor/src/glad.c",
            "vendor/src/integrator_janus.c",
            "vendor/src/transformations.c",
        ])
        .define("LIBREBOUND", "1")
        .include("vendor/src")
        .warnings(false)
        .compile("rebound");

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("reb_.*")
        .allowlist_type("reb_.*")
        .allowlist_var("reb_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("writing bindings to {}", out_path.to_str().unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
