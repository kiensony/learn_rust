#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub name: String,
    pub score: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RowError {
    EmptyName,
    BadScore,
}

pub fn parse_row(line: &str) -> Result<Row, RowError> {
    let _ = line;
    Err(RowError::BadScore)
}

pub fn passing(rows: impl IntoIterator<Item = Row>, min: u8) -> Vec<String> {
    let _ = (rows, min);
    Vec::new()
}
