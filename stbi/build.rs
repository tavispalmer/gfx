use std::{env, path::PathBuf, process::Command};

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    
    // these args are from running meson + ninja
    assert!(Command::new(env::var_os("CC").unwrap_or("cc".into()))
        .arg("-Isrc")
        .arg("-fdiagnostics-color=always")
        .arg("-Wall")
        .arg("-Winvalid-pch")
        .arg("-O3") // we always compile stb_image.h in release mode

        // we use callbacks to use rust's stdio instead
        .arg("-DSTBI_NO_STDIO")
        // define STB_IMAGE_IMPLEMENTATION so we get the function definitions
        .arg("-DSTB_IMAGE_IMPLEMENTATION")
        // need to define c language explicitly, since we're compiling a header file
        .arg("-xc")

        .arg("-MD")
        .arg("-MQ")
        .arg(out.join("stb_image.o"))
        .arg("-MF")
        .arg(out.join("stb_image.o.d"))
        .arg("-o")
        .arg(out.join("stb_image.o"))
        .arg("-c")
        .arg("stb_image.h")
        .status()
        .unwrap()
        .success());

    // create library
    assert!(Command::new("rm")
        .arg("-f")
        .arg(out.join("libstbi.a"))
        .status()
        .unwrap()
        .success());

    assert!(Command::new("ar")
        .arg("csr")
        .arg(out.join("libstbi.a"))
        .arg(out.join("stb_image.o"))
        .status()
        .unwrap()
        .success());

    assert!(Command::new("ranlib")
        .arg("-c")
        .arg(out.join("libstbi.a"))
        .status()
        .unwrap()
        .success());

    // link with stbi
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-lib=stbi");
}
