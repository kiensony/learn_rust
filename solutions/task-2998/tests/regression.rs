use task_2998::*;

#[test]
fn transfer_case() {
    assert_eq!(read_versioned(&[1, 0, 0, 0]), Err("length"));
}

#[test]
fn the_same_bytes_keep_the_original_meaning_on_each_version() {
    assert_eq!(read_versioned(&[1, 0x12, 0x34]), Ok(0x3412));
    assert_eq!(read_versioned(&[2, 0x12, 0x34]), Ok(0x1234));
    assert_eq!(read_versioned(&[3]), Err("length"));
}
