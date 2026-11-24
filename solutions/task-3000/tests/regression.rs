use task_3000::*;

#[test]
fn transfer_case() {
    assert_eq!(affected_ticks(&[(0, 65535), (0, 65535)]), 65535);
}

#[test]
fn every_pair_of_small_intervals_agrees_with_counting_ticks() {
    for a in 0..=4 {
        for b in 0..=4 {
            for c in 0..=4 {
                for d in 0..=4 {
                    let incidents = [(a, b), (c, d)];
                    let oracle = (0..4)
                        .filter(|&t| incidents.iter().any(|&(start, end)| start <= t && t < end))
                        .count() as u32;
                    assert_eq!(affected_ticks(&incidents), oracle);
                }
            }
        }
    }
}
