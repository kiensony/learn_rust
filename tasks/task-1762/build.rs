fn main() {
    println!("cargo::rerun-if-changed=units.txt");
    let units = std::fs::read_to_string("units.txt").unwrap();
    println!("cargo::rustc-env=TRACK_UNITS={}", units.trim());
}
