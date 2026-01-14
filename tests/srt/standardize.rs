use super::*;

#[test]
fn already_standard() {
    assert_eq!(new_standard(), new_standard().standardize().unwrap());
}

#[test]
fn start_blanks() {
    let mut srt = new_standard();
    for _ in 0..4 {
        srt.insert(0, SrtLine::new(""));
    }
    assert_eq!(new_standard(), srt.standardize().unwrap());
}

#[test]
fn ty_line_after_time_range() {
    let mut srt = new_standard();
    srt[2].ty = SrtLineType::Number;
    assert_eq!(new_standard(), srt.standardize().unwrap());
}

#[test]
fn missing_number() {
    let mut srt = new_standard();
    srt.remove(0);
    let standardized = srt.standardize().unwrap();
    let new = new_standard();
    assert_eq!(new[0].ty, standardized[0].ty);
    assert_eq!(&new[1..], &standardized[1..]);
}

#[test]
fn missing_end_blank() {
    let mut srt = new_standard();
    srt.pop();
    let standardized = srt.standardize().unwrap();
    let new = new_standard();
    assert_eq!(new[3].ty, standardized[3].ty);
    assert_eq!(&new[..3], &standardized[..3]);
}

#[test]
fn time_tange_text_pair() {
    let mut srt = new_standard();
    srt.pop();
    srt.remove(0);
    let standardized = srt.standardize().unwrap();
    let new = new_standard();
    assert_eq!(new[0].ty, standardized[0].ty);
    assert_eq!(new[3].ty, standardized[3].ty);
    assert_eq!(&new[1..3], &standardized[1..3]);
}

#[test]
fn remove_non_standard() {
    let mut srt = new_standard();
    srt.push(SrtLine::new("x\n"));
    assert_eq!(new_standard(), srt.standardize().unwrap());
}

#[test]
fn sort_blocks_by_times() {
    let mut srt = new_standard();
    srt.push(SrtLine::new("00:00:00,000 --> 00:00:02,000\n"));
    srt.push(SrtLine::new("x\n"));
    let srt = srt.standardize().unwrap();
    assert_eq!(
        srt[1].ty,
        SrtLineType::TimeRange((
            Time::new_unchecked(0, 0, 0, 0),
            Time::new_unchecked(0, 0, 2, 0)
        ))
    );
    assert_eq!(
        srt[5].ty,
        SrtLineType::TimeRange((
            Time::new_unchecked(0, 0, 0, 0),
            Time::new_unchecked(0, 0, 5, 0)
        ))
    );
}
