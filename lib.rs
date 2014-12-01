//! A library for decoding JIS X 0208-encoded characters into Unicode characters,
//! as specified by the the Unicode Consortium's JIS0208.TXT.

mod map;

/// Decode a single JIS X 0208 codepoint. Returns None if no mapping exists.
pub fn decode_codepoint(codepoint: u16) -> Option<char> {
    map::decode(codepoint)
}

/// Encode a single Unicode codepoint. Returns None if no mapping exists.
pub fn encode_codepoint(codepoint: char) -> Option<u16> {
    map::encode(codepoint)
}

#[test]
fn test_decode() {
    assert_eq!(decode_codepoint(0x2341), std::char::from_u32(0xff21));
    assert_eq!(decode_codepoint(0x3000), None);
}

#[test]
fn test_encode() {
    assert_eq!(encode_codepoint(std::char::from_u32(0xff21).unwrap()), Some(0x2341));
    assert_eq!(encode_codepoint('a'), None);
}
