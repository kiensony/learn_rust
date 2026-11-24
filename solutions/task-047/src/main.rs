fn main() {
    let label = std::env::args().nth(1).unwrap_or_default();
    let border = task_047::divider();
    println!("{border}\n{label}\n{border}");
}
