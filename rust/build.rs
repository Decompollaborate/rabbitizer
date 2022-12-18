fn main() {
    let mut c_paths: Vec<std::path::PathBuf> = glob::glob("../src/**/*.c").unwrap().filter_map(|g| g.ok()).collect();
    let mut h_paths: Vec<std::path::PathBuf> = glob::glob("../include/**/*.h").unwrap().filter_map(|g| g.ok()).collect();

    // Needed because `cargo publish` is dumb
    let mut c_paths_2: Vec<std::path::PathBuf> = glob::glob("../../../../src/**/*.c").unwrap().filter_map(|g| g.ok()).collect();
    let mut h_paths_2: Vec<std::path::PathBuf> = glob::glob("../../../../include/**/*.h").unwrap().filter_map(|g| g.ok()).collect();

    c_paths.append(&mut c_paths_2);
    h_paths.append(&mut h_paths_2);

    for path in c_paths.iter().chain(&h_paths) {
        println!("cargo:rerun-if-changed={}", path.to_string_lossy());
    }

    assert!(c_paths.len() > 0);

    cc::Build::new()
        .files(c_paths)
        .include("../include")
        .include("../../../../include")
        .warnings(false)
        .compile("rabbitizer");
}
