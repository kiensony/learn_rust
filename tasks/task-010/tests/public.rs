use task_010::{parse_row, passing, Row, RowError};

#[test]
fn parse_ok() {
    assert_eq!(
        parse_row("Ada=90"),
        Ok(Row {
            name: "Ada".into(),
            score: 90
        })
    );
}

#[test]
fn parse_empty_name() {
    assert_eq!(parse_row("=10"), Err(RowError::EmptyName));
}

#[test]
fn passing_names() {
    let rows = vec![
        Row {
            name: "Bea".into(),
            score: 50,
        },
        Row {
            name: "Ada".into(),
            score: 90,
        },
    ];
    assert_eq!(passing(rows, 60), vec!["Ada".to_string()]);
}
