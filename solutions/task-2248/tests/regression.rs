use task_2248::*;

#[test]
fn transfer_case() {
    assert_eq!(
        borrowed_scalars_rolling_load("a\nblue\nblue\r\nz\n a\nblue\nblue\r\nz\n", 3),
        vec![
            3u64, 6u64, 9u64, 12u64, 15u64, 18u64, 21u64, 24u64, 27u64, 30u64, 33u64, 36u64, 39u64,
            42u64, 45u64, 48u64, 51u64, 54u64, 57u64, 60u64, 63u64, 66u64, 69u64, 72u64, 75u64,
            78u64, 81u64, 84u64, 87u64
        ]
    );
}
