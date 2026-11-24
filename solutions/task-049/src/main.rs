fn main() {
    let count = std::env::args().skip(1).count();
    println!("arguments={count}");
}
