# task-010 — Grade sheet

```rust
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

pub fn parse_row(line: &str) -> Result<Row, RowError>
pub fn passing(rows: impl IntoIterator<Item = Row>, min: u8) -> Vec<String>
```

`parse_row` format: `name=score` (one `=`).

- Trim name and score sides.
- Empty name → `Err(RowError::EmptyName)`
- Score not a `u8` in `0..=100` → `Err(RowError::BadScore)`
- Extra `=` → `BadScore`
- Missing `=` → `BadScore`

`passing`:

- Keep rows with `score >= min`
- Return names sorted ascending
- Use iterator adapters (`filter`, `map`, `collect`). A hand-rolled `for` that does the same is weaker; review will mark it.

No `unsafe`.
