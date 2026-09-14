# Review — task-006

## Must

- Proof + `TRACK_PROOF == 0x52535400 | 6`
- `Option` on `new`, no panic
- Derives present
- No `unsafe`

## Quality

- Methods on `impl Rect`, not free functions only
- No wrapping mul tricks

## Reject

- Missing proof
- Zero-size rectangles allowed
- Extra builders not asked for
