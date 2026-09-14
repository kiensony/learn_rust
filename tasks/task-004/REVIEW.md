# Review — task-004

## Must

- Proof + `TRACK_PROOF == 0x52535400 | 4`
- Signature takes `String`, not `&str`
- ASCII uppercase + `id:` prefix
- No `unsafe`

## Quality

- Prefer mutating the owned `String`
- No `clone` of `name`

## Reject

- Missing proof
- Immediate `clone` / double String
- Changing the signature
