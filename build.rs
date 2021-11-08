fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/llvm-bindings.cpp")
        .compile("llvm-bindings");
    println!("cargo:rustc-link-lib=LLVM-12");
    println!("cargo:rerun-if-changed=src/llvm-bindings.cpp");
}
