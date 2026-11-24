fn main() {
    let mut input = vec![0u8; 128 * 1024];
    for (i, byte) in input.iter_mut().enumerate() {
        *byte = (i % 251) as u8;
    }
    let start = std::time::Instant::now();
    for _ in 0..1024 {
        std::hint::black_box(task_2284::find_newlines(std::hint::black_box(&input)));
    }
    eprintln!("elapsed {:?}", start.elapsed());
}
