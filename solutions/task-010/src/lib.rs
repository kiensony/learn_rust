// rust-track: handwritten task-010
pub const TRACK_PROOF: u32 = 0x52535400 | 10;

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
    let (name, score) = line.split_once('=').ok_or(RowError::BadScore)?;
    if score.contains('=') {
        return Err(RowError::BadScore);
    }
    let name = name.trim();
    if name.is_empty() {
        return Err(RowError::EmptyName);
    }
    let score = score.trim().parse::<u8>().map_err(|_| RowError::BadScore)?;
    if score > 100 {
        return Err(RowError::BadScore);
    }
    Ok(Row {
        name: name.into(),
        score,
    })
}
pub fn passing(rows: impl IntoIterator<Item = Row>, min: u8) -> Vec<String> {
    let mut names: Vec<_> = rows
        .into_iter()
        .filter(|row| row.score >= min)
        .map(|row| row.name)
        .collect();
    names.sort();
    names
}
