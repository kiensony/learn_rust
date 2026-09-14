# Review — task-008

## Must

- Proof + `TRACK_PROOF == 0x52535400 | 8`
- `BTreeMap<String, usize>`
- ASCII lower, whitespace split
- No `unsafe`

## Quality

- `entry` API or `get_mut`, not repeated contains+insert clones beyond the key
- No regex crate

## Reject

- Missing proof
- `HashMap` instead of `BTreeMap`
- Tokenizing on punctuation when the spec says whitespace only
