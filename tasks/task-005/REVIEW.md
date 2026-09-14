# Review — task-005

## Must

- Proof + `TRACK_PROOF == 0x52535400 | 5`
- Returns `&str`, no owned String
- Tie → first argument
- Explicit lifetime on the fn

## Quality

- Compare `.len()`, not chars unless needed (bytes are specified)
- No `clone` / `to_owned`

## Reject

- Missing proof
- Allocating a new String and leaking / `Box::leak`
- `unsafe`
