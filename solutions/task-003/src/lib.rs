// rust-track: handwritten task-003
pub const TRACK_PROOF: u32 = 0x52535400 | 3;

pub fn parse_pair(input: &str) -> Result<(i32, i32), String> {
    let (left, right) = input.split_once(':').ok_or("missing colon")?;
    if right.contains(':') {
        return Err("too many colons".into());
    }
    let (left, right) = (left.trim(), right.trim());
    if left.is_empty() || right.is_empty() {
        return Err("empty side".into());
    }
    let number = |field: &str| field.parse::<i32>().map_err(|_| String::from("not an i32"));
    Ok((number(left)?, number(right)?))
}
