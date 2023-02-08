//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    #[allow(clippy::if_same_then_else)]
    let buf: &[u8] = if env::var_os("CARGO_FEATURE_MICROBIT_V2").is_some() {
        include_bytes!("memory_microbit_v2.x")
    } else if env::var_os("CARGO_FEATURE_PARTICLE_XENON").is_some() {
        include_bytes!("memory_particle_xenon.x")
    } else {
        panic!("Unknown target - cannot determine memory.x - aborting")
    };
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(buf)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");
}
