// Returns a byte slice with leading and trailing whitespace removed.
pub fn trim(data: &[u8]) -> &[u8] {
    let mut start = 0;
    let mut end = data.len();

    while start < end && data[start].is_ascii_whitespace() {
        start += 1;
    }

    while end > start && data[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    &data[start..end]
}

pub fn trim_start(data: &[u8]) -> &[u8] {
    let mut pos = 0;
    while pos < data.len() && data[pos].is_ascii_whitespace() {
        pos += 1;
    }
    &data[pos..]
}

pub fn trim_end(data: &[u8]) -> &[u8] {
    let mut end = data.len();
    while end > 0 && data[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    &data[0..end]
}

pub fn trim_bom(data: &[u8]) -> &[u8] {
    data.strip_prefix(crate::BOM).unwrap_or(data)
}
