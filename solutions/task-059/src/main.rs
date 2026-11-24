fn main() {
    let label = std::env::args().nth(1).unwrap_or_default();
    println!("Item #{label}");
    print!("{}", include_str!("../caption.txt"));
}
