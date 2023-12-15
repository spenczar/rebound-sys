use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("./vendor/rebound/src")
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
            "vendor/rebound/src/binarydiff.c",
            "vendor/rebound/src/boundary.c",
            "vendor/rebound/src/collision.c",
            "vendor/rebound/src/communication_mpi.c",
            "vendor/rebound/src/derivatives.c",
            "vendor/rebound/src/display.c",
	    "vendor/rebound/src/fmemopen.c",
            "vendor/rebound/src/glad.c",
            "vendor/rebound/src/gravity.c",
            "vendor/rebound/src/input.c",
            "vendor/rebound/src/integrator.c",
            "vendor/rebound/src/integrator_bs.c",
            "vendor/rebound/src/integrator_eos.c",
            "vendor/rebound/src/integrator_ias15.c",
            "vendor/rebound/src/integrator_janus.c",
            "vendor/rebound/src/integrator_leapfrog.c",
            "vendor/rebound/src/integrator_mercurius.c",
            "vendor/rebound/src/integrator_saba.c",
            "vendor/rebound/src/integrator_sei.c",
            "vendor/rebound/src/integrator_whfast.c",
            "vendor/rebound/src/integrator_whfast512.c",
            "vendor/rebound/src/output.c",
            "vendor/rebound/src/particle.c",
            "vendor/rebound/src/rebound.c",
            "vendor/rebound/src/rotations.c",
            "vendor/rebound/src/simulationarchive.c",
            "vendor/rebound/src/tools.c",
            "vendor/rebound/src/transformations.c",
            "vendor/rebound/src/tree.c",
        ])
        .define("LIBREBOUND", "1")
	.define("_GNU_SOURCE", None)
        .include("vendor/rebound/src")
        .warnings(false)
        .compile("rebound");

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("reb_.*")
        .allowlist_type("reb_.*")
        .allowlist_type("REB_.*")
        .allowlist_var("reb_.*")
        .allowlist_var("REB_.*")
        .allowlist_type("RADAU")
        .allowlist_type("DHEM")
        .allowlist_type("_?controlVars")
        .allowlist_type("UNIVERSAL_VARS")
        .allowlist_type("_?StumpfCoefficients")
        .allowlist_recursively(false)
        .raw_line("use libc::{pthread_mutex_t,FILE,pthread_t};")
        .raw_line("type sig_atomic_t = std::os::raw::c_int;")
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("writing bindings to {}", out_path.to_str().unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
