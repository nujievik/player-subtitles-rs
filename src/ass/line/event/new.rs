use super::{Event, EventType, format::EventFormatPositions};
use crate::{Time, byte_helpers};

impl<'a> Event<'a> {
    /// Gets `Event` from formatted byte line.
    pub fn get_new(line: &'a [u8], positions: &EventFormatPositions) -> Option<Self> {
        let (ty, mut remainder) = get_event_type_and_trim_line(line)?;

        let mut parts = [b"".as_slice(); 9];
        for i in 0..9 {
            let pos = remainder.iter().position(|&b| b == b',')?;
            parts[i] = &remainder[..pos];
            remainder = if remainder.len() > pos + 1 {
                byte_helpers::trim_start(&remainder[pos + 1..])
            } else {
                return None;
            }
        }
        parts[0] = byte_helpers::trim_start(parts[0]);

        Some(Self {
            bytes: line,
            ty,
            layer: byte_helpers::get_u16(parts[positions.layer() as usize])?,
            start: get_time(parts[positions.start() as usize])?,
            end: get_time(parts[positions.end() as usize])?,
            style_name: parts[positions.style_name() as usize],
            character_name: parts[positions.character_name() as usize],
            margin_l: byte_helpers::get_u16(parts[positions.margin_l() as usize])?,
            margin_r: byte_helpers::get_u16(parts[positions.margin_r() as usize])?,
            margin_v: byte_helpers::get_u16(parts[positions.margin_v() as usize])?,
            effect: parts[positions.effect() as usize],
            text: remainder,
        })
    }

    pub(crate) const fn new() -> Self {
        Self {
            bytes: &[],
            ty: EventType::Dialogue,
            layer: 0,
            start: Time::new_unchecked(0, 0, 0, 0),
            end: Time::new_unchecked(0, 0, 0, 0),
            style_name: b"Default",
            character_name: &[],
            margin_l: 0,
            margin_r: 0,
            margin_v: 0,
            effect: &[],
            text: &[],
        }
    }
}

fn get_event_type_and_trim_line<'a>(line: &'a [u8]) -> Option<(EventType<'a>, &'a [u8])> {
    let mut pos = 0usize;
    while pos < line.len() {
        match line[pos] {
            b',' => return None,
            b':' => {
                if !(pos + 1 < line.len()) {
                    return None;
                }

                let ty = match &line[..pos] {
                    b"Dialogue" => EventType::Dialogue,
                    b"Comment" => EventType::Comment,
                    b"Picture" => EventType::Picture,
                    b"Sound" => EventType::Sound,
                    b"Movie" => EventType::Movie,
                    b"Command" => EventType::Command,
                    _ => EventType::Unrecognized(&line[..=pos]),
                };

                return Some((ty, &line[pos + 1..]));
            }
            _ => pos += 1,
        }
    }
    None
}

fn get_time(data: &[u8]) -> Option<Time> {
    let mut hhs_it = data.split(|b| matches!(b, b'.'));
    let data = hhs_it.next().unwrap();
    let mut it = data.split(|b| matches!(b, b':')).rev();

    let hundredths = if let Some(x) = it.next() {
        x
    } else {
        it.next()?
    };
    let secs = it.next()?;
    let mins = it.next()?;
    let hours = it.next()?;

    let millis = byte_helpers::get_u16(hundredths)? * 10;
    let secs = byte_helpers::get_u8(secs)?;
    let mins = byte_helpers::get_u8(mins)?;
    let hours = byte_helpers::get_u16(hours)?;

    Time::new(hours, mins, secs, millis).ok()
}
