use super::*;

#[test]
fn is_blank() {
    let blank = SrtLine::new("\n");
    assert!(SrtBlock(&[blank]).is_blank());

    [
        SrtLineType::Number,
        SrtLineType::Text,
        SrtLineType::TimeRange(Default::default()),
    ]
    .into_iter()
    .for_each(|ty| {
        let l = line::new_with_ty("", ty);
        assert!(!SrtBlock(&[l]).is_blank());
    })
}

#[test]
fn is_standard() {
    let mut srt = super::new_standard();
    assert!(SrtBlock(&srt[..]).is_standard());

    srt[3].ty = SrtLineType::Text;
    assert!(!SrtBlock(&srt[..]).is_standard());
    srt[3].ty = SrtLineType::Blank;

    for end in 0..3 {
        assert!(!SrtBlock(&srt[..end]).is_standard());
    }
}

#[test]
fn validate_standard() {
    let mut srt = super::new_standard();
    SrtBlock(&srt[..]).validate_standard().unwrap();

    srt[3].ty = SrtLineType::Text;
    SrtBlock(&srt[..]).validate_standard().unwrap_err();
    srt[3].ty = SrtLineType::Blank;

    for end in 0..3 {
        SrtBlock(&srt[..end]).validate_standard().unwrap_err();
    }
}
