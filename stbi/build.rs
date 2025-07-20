fn main() {
    println!("cargo::rerun-if-changed=stb_image.c");
    println!("cargo::rerun-if-changed=stb_image.h");
    
    cc::Build::new()
        .file("stb_image.c")
        .compile("stbi");
}
