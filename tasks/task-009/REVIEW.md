# Review — task-009

## Must

- Proof + `TRACK_PROOF == 0x52535400 | 9`
- Private storage
- Trait bound only on `summarize_top` impl
- `pop` returns `Option`

## Quality

- `Vec<T>` inside is fine
- No `clone` of T unless T: Clone is forced (it is not)

## Reject

- Missing proof
- Public `vec` field
- `unwrap` on empty pop
- `unsafe`
