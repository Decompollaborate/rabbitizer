fn main() {
    println!("cargo:rustc-link-search=../build/");
    println!("cargo:rustc-link-search=build/");
}
