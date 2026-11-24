fn main() {
    let label = std::env::args().nth(1).unwrap_or_default();
    let label = "sample";
    println!("{label}");
}
