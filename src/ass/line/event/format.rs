use crate::byte_helpers;

// Contains positions of Event fields.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EventFormat<'a> {
    bytes: &'a [u8],
    positions: EventFormatPositions,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct EventFormatPositions {
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

impl<'a> EventFormat<'a> {
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes
    }

    pub fn positions(&self) -> &EventFormatPositions {
        &self.positions
    }
}

macro_rules! field_method {
    ($field:ident) => {
        #[doc = concat!("Returns `", stringify!($field), "` position.")]
        pub const fn $field(&self) -> u8 {
            self.$field
        }
    };
}

impl EventFormatPositions {
    field_method!(layer);
    field_method!(start);
    field_method!(end);
    field_method!(style_name);
    field_method!(character_name);
    field_method!(margin_l);
    field_method!(margin_r);
    field_method!(margin_v);
    field_method!(effect);
}

impl<'a> EventFormat<'a> {
    pub fn new() -> EventFormat<'a> {
        Self {
            bytes:
                b"Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text",
            positions: EventFormatPositions::new(),
        }
    }

    /// Gets `EventFormat` from format byte line.
    pub fn get_new(line: &'a [u8]) -> Option<EventFormat<'a>> {
        let remainder = if line.len() < 16 || !line.starts_with(b"Format:") {
            return None;
        } else {
            byte_helpers::trim_start(&line[7..])
        };

        let mut comma_count = 0u8;
        let mut it = remainder
            .split(|b| matches!(b, b','))
            .map(|part| byte_helpers::trim_start(part));
        let mut positions = EventFormatPositions::new();

        while let Some(x) = it.next() {
            let field = match x {
                b"Layer" => &mut positions.layer,
                b"Start" => &mut positions.start,
                b"End" => &mut positions.end,
                b"Style" => &mut positions.style_name,
                b"Name" => &mut positions.character_name,
                b"MarginL" => &mut positions.margin_l,
                b"MarginR" => &mut positions.margin_r,
                b"MarginV" => &mut positions.margin_v,
                b"Effect" => &mut positions.effect,
                _ => return None,
            };
            *field = comma_count;

            comma_count += 1;
            if comma_count == EventFormatPositions::NUMBER_OF_FIELDS {
                break;
            }
        }

        (comma_count >= EventFormatPositions::NUMBER_OF_FIELDS).then(|| EventFormat {
            bytes: line,
            positions,
        })
    }
}

impl EventFormatPositions {
    pub(crate) const NUMBER_OF_FIELDS: u8 = 9;

    pub fn new() -> EventFormatPositions {
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
}
