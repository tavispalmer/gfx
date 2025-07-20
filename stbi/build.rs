use std::{env, ffi::{OsStr, OsString}, path::PathBuf, process::Command};

fn concat<A: AsRef<OsStr>, B: AsRef<OsStr>>(a: A, b: B) -> OsString {
    let a = a.as_ref();
    let b = b.as_ref();
    let mut ret = OsString::with_capacity(a.len() + b.len());
    ret.push(a);
    ret.push(b);
    ret
}

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    #[cfg(target_os = "windows")]
    if Command::new("cl").arg("/?").status().is_ok_and(|status| status.success()) {
        assert!(Command::new("cl")
            .arg("/MD")
            .arg("/nologo")
            .arg("/showIncludes")
            .arg("/utf-8")
            .arg("/W2")
            .arg("/O2")
            .arg("/Gw")
            .arg(concat("/Fd", out.join("app.pdb")))
            .arg(concat("/Fo", out.join("app.obj")))
            .arg("/c")
            .arg("stb_image.c")
            .status()
            .unwrap()
            .success());

        assert!(Command::new("lib")
            .arg("/NOLOGO")
            .arg("/MACHINE:x86")
            .arg(concat("/OUT:", out.join("libapp.a")))
            .arg(out.join("app.obj"))
            .status()
            .unwrap()
            .success());
    }
    
    #[cfg(not(target_os = "windows"))]
    if Command::new("cc").arg("--version").status().is_ok_and(|status| status.success()) {
        // these args are from running meson + ninja
        assert!(Command::new("cc")
            .arg("-fdiagnostics-color=always")
            .arg("-Wall")
            .arg("-Winvalid-pch")
            .arg("-O3") // we always compile stb_image.h in release mode
            .arg("-MD")
            .arg("-MQ")
            .arg(out.join("stb_image.o"))
            .arg("-MF")
            .arg(out.join("stb_image.o.d"))
            .arg("-o")
            .arg(out.join("stb_image.o"))
            .arg("-c")
            .arg("stb_image.c")
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
    }

    // link with stbi
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-lib=stbi");
}
