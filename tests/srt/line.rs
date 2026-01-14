use super::*;
use std::borrow::Cow;

pub const fn new_with_ty(s: &'static str, ty: SrtLineType) -> SrtLine<'static> {
    SrtLine {
        raw: Cow::Borrowed(s.as_bytes()),
        ty,
    }
}

#[test]
fn new_blank() {
    ["", " ", "\n", "\t"].iter().for_each(|s| {
        let line = SrtLine::new(s);
        assert_eq!(s.as_bytes(), &*line.raw);
        assert_eq!(SrtLineType::Blank, line.ty);
    })
}

#[test]
fn new_number() {
    ["0", "4", " 0 ", "4\n", "\t4\n"].iter().for_each(|s| {
        let line = SrtLine::new(s);
        assert_eq!(s.as_bytes(), &*line.raw);
        assert_eq!(SrtLineType::Number, line.ty);
    })
}

#[test]
fn new_time_range() {
    [
        (
            "00:00:00,000 --> 00:00:05,000",
            Time::new_unchecked(0, 0, 0, 0),
            Time::new_unchecked(0, 0, 5, 0),
        ),
        (
            "12:45:55,657 --> 87:55:44,321",
            Time::new_unchecked(12, 45, 55, 657),
            Time::new_unchecked(87, 55, 44, 321),
        ),
    ]
    .into_iter()
    .for_each(|(s, start, end)| {
        let line = SrtLine::new(s);
        assert_eq!(s.as_bytes(), &*line.raw);
        assert_eq!(SrtLineType::TimeRange((start, end)), line.ty);
    })
}

#[test]
fn new_text() {
    ["x", "abc", " qwerty ", "text\n", "\tdef\n"]
        .iter()
        .for_each(|s| {
            let line = SrtLine::new(s);
            assert_eq!(s.as_bytes(), &*line.raw);
            assert_eq!(SrtLineType::Text, line.ty);
        })
}

#[test]
fn try_new_blank() {
    ["\n", "\r", "\t\n"].iter().for_each(|s| {
        let line = SrtLine::try_new(s).unwrap();
        assert_eq!(s.as_bytes(), &*line.raw);
        assert_eq!(SrtLineType::Blank, line.ty);
    });

    ["", " ", "\n\n"].iter().for_each(|s| {
        SrtLine::try_new(s).unwrap_err();
    })
}

#[test]
fn try_new_number() {
    ["0\n", "4\r", "\t4\n"].iter().for_each(|s| {
        let line = SrtLine::try_new(s).unwrap();
        assert_eq!(s.as_bytes(), &*line.raw);
        assert_eq!(SrtLineType::Number, line.ty);
    });

    ["0", "4", "4\n\n"].iter().for_each(|s| {
        SrtLine::try_new(s).unwrap_err();
    });
}

#[test]
fn try_new_time_range() {
    [
        (
            "00:00:00,000 --> 00:00:05,000\n",
            Time::new_unchecked(0, 0, 0, 0),
            Time::new_unchecked(0, 0, 5, 0),
        ),
        (
            "12:45:55,657 --> 87:55:44,321\r",
            Time::new_unchecked(12, 45, 55, 657),
            Time::new_unchecked(87, 55, 44, 321),
        ),
    ]
    .into_iter()
    .for_each(|(s, start, end)| {
        let line = SrtLine::try_new(s).unwrap();
        assert_eq!(s.as_bytes(), &*line.raw);
        assert_eq!(SrtLineType::TimeRange((start, end)), line.ty);
    });

    [
        "00:00:00,000 --> 00:00:05,000",
        "00:00:00,000 --> 00:00:05,000\n\n",
        "00:00:05,000 --> 00:00:00,000\n",
        "100:00:00,000 --> 100:00:05,000\n",
    ]
    .into_iter()
    .for_each(|s| {
        SrtLine::try_new(s).unwrap_err();
    })
}

#[test]
fn try_new_text() {
    ["x\n", "abc\r", " qwerty \n", "text\n", "\tdef\n"]
        .iter()
        .for_each(|s| {
            let line = SrtLine::try_new(s).unwrap();
            assert_eq!(s.as_bytes(), &*line.raw);
            assert_eq!(SrtLineType::Text, line.ty);
        });

    ["x", "abc", " qwerty ", "text\n\n"].iter().for_each(|s| {
        SrtLine::try_new(s).unwrap_err();
    })
}
