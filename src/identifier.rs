pub(crate) const fn is_ascii_letter(byte: u8) -> bool {
    (byte >= b'a' && byte <= b'z') || (byte >= b'A' && byte <= b'Z')
}

pub(crate) const fn is_checked_identifier(identifier: &str) -> bool {
    let bytes = identifier.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if !is_ascii_letter(bytes[0]) && bytes[0] != b'_' {
        return false;
    }
    let mut index = 1;
    while index < bytes.len() {
        let byte = bytes[index];
        if !(is_ascii_letter(byte)
            || (byte >= b'0' && byte <= b'9')
            || byte == b'-'
            || byte == b'_')
        {
            return false;
        }
        index += 1;
    }
    true
}

pub(crate) const fn is_checked_custom_property_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() < 3 || bytes[0] != b'-' || bytes[1] != b'-' {
        return false;
    }
    if !is_ascii_letter(bytes[2]) && bytes[2] != b'_' {
        return false;
    }
    let mut index = 3;
    while index < bytes.len() {
        let byte = bytes[index];
        if !(is_ascii_letter(byte)
            || (byte >= b'0' && byte <= b'9')
            || byte == b'-'
            || byte == b'_')
        {
            return false;
        }
        index += 1;
    }
    true
}
