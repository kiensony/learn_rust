use task_010::{parse_row, passing, Row, RowError, TRACK_PROOF};

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52535400 | 10);
}

#[test]
fn trim() {
    assert_eq!(
        parse_row("  Ada = 7 "),
        Ok(Row {
            name: "Ada".into(),
            score: 7
        })
    );
}

#[test]
fn bounds() {
    assert!(parse_row("A=0").is_ok());
    assert!(parse_row("A=100").is_ok());
    assert_eq!(parse_row("A=101"), Err(RowError::BadScore));
    assert_eq!(parse_row("A=-1"), Err(RowError::BadScore));
}

#[test]
fn extra_eq() {
    assert_eq!(parse_row("A=1=2"), Err(RowError::BadScore));
    assert_eq!(parse_row("A"), Err(RowError::BadScore));
}

#[test]
fn passing_sort_and_min() {
    let rows = vec![
        Row {
            name: "Zed".into(),
            score: 80,
        },
        Row {
            name: "Amy".into(),
            score: 80,
        },
        Row {
            name: "Low".into(),
            score: 79,
        },
    ];
    assert_eq!(passing(rows, 80), vec!["Amy".to_string(), "Zed".to_string()]);
}
