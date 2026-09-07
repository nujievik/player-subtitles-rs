use crate::{Time, byte_helpers};

// Contains positions of Event fields.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EventFormat {
    layer: u8,
    start: u8,
    end: u8,
    style_name: u8,
    character_name: u8,
    margin_l: u8,
    margin_r: u8,
    margin_v: u8,
    effect: u8,
}

#[derive(Debug, PartialEq)]
pub struct Event<'a> {
    pub(crate) ty: EventType<'a>,
    // Subtitles having different layer number will be ignored during the collusion detection.
    // Higher numbered layers will be drawn over the lower numbered.
    pub(crate) layer: u16,
    pub(crate) start: Time,
    pub(crate) end: Time,
    pub(crate) style_name: &'a [u8],
    // Character name. This is the name of the character who speaks the dialogue. It is for
    // information only, to make the script is easier to follow when editing/timing.
    pub(crate) character_name: &'a [u8],
    pub(crate) margin_l: u16,
    pub(crate) margin_r: u16,
    pub(crate) margin_v: u16,
    pub(crate) effect: Effect,
    // Subtitle Text. This is the actual text which will be displayed as a subtitle onscreen.
    // Everything after the 9th comma is treated as the subtitle text, so it can include commas.
    // The text can include \n codes which is a line break, and can include Style Override control
    // codes, which appear between braces { }.
    pub(crate) text: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub enum EventType<'a> {
    Dialogue,
    Comment,
    Picture,
    Sound,
    Movie,
    Command,
    Unrecognized(&'a [u8]),
}

#[derive(Debug, PartialEq)]
pub enum Effect {
    Empty,
    Karaoke,
    ScrollUp,
    Banner,
}

impl EventFormat {
    pub(crate) const fn new() -> Self {
        Self {
            layer: 0,
            start: 1,
            end: 2,
            style_name: 3,
            character_name: 4,
            margin_l: 5,
            margin_r: 6,
            margin_v: 7,
            effect: 8,
        }
    }

    pub(crate) fn get_new(line: &[u8]) -> Option<Self> {
        let remainder = if line.len() < 16 || !line.starts_with(b"Format:") {
            return None;
        } else {
            byte_helpers::trim_start(&line[7..])
        };

        let mut comma_count = 0u8;
        let mut it = remainder
            .split(|b| matches!(b, b','))
            .map(|part| byte_helpers::trim_start(part));
        let mut format = EventFormat::new();

        while let Some(x) = it.next() {
            let field = match x {
                b"Layer" => &mut format.layer,
                b"Start" => &mut format.start,
                b"End" => &mut format.end,
                b"Style" => &mut format.style_name,
                b"Name" => &mut format.character_name,
                b"MarginL" => &mut format.margin_l,
                b"MarginR" => &mut format.margin_r,
                b"MarginV" => &mut format.margin_v,
                b"Effect" => &mut format.effect,
                _ => return None,
            };
            *field = comma_count;

            comma_count += 1;
            if comma_count == 9 {
                break;
            }
        }

        if comma_count < 9 { None } else { Some(format) }
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        b"Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text"
    }
}

impl<'a> Event<'a> {
    pub(crate) const fn new() -> Self {
        Self {
            ty: EventType::Dialogue,
            layer: 0,
            start: Time::new_unchecked(0, 0, 0, 0),
            end: Time::new_unchecked(0, 0, 0, 0),
            style_name: b"Default",
            character_name: &[],
            margin_l: 0,
            margin_r: 0,
            margin_v: 0,
            effect: Effect::Empty,
            text: &[],
        }
    }

    pub(crate) fn get_new(line: &'a [u8], format: EventFormat) -> Option<Self> {
        let (ty, mut remainder) = get_event_type_and_trim_line(line)?;
        dbg!("ty ok");

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
            ty,
            layer: byte_helpers::get_u16(parts[format.layer as usize])?,
            start: get_time(parts[format.start as usize])?,
            end: get_time(parts[format.end as usize])?,
            style_name: parts[format.style_name as usize],
            character_name: parts[format.character_name as usize],
            margin_l: byte_helpers::get_u16(parts[format.margin_l as usize])?,
            margin_r: byte_helpers::get_u16(parts[format.margin_r as usize])?,
            margin_v: byte_helpers::get_u16(parts[format.margin_v as usize])?,
            effect: Effect::Empty,
            text: remainder,
        })
    }
}

impl<'a> EventType<'a> {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Dialogue => b"Dialogue",
            Self::Comment => b"Comment",
            Self::Picture => b"Picture",
            Self::Sound => b"Sound",
            Self::Movie => b"Movie",
            Self::Command => b"Command",
            Self::Unrecognized(bs) => bs,
        }
    }
}

impl Effect {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        match self {
            Effect::Karaoke => b"Karaoke",
            _ => &[],
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
