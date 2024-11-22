use ring_tag::Tag;

/// Creates tag identifying javascript code
pub fn js_tag() -> Tag {
    Tag::from("js").with_color((0xf7, 0xdf, 0x1e))
}

/// Creates tag identifying typescript code
pub fn ts_tag() -> Tag {
    Tag::from("ts").with_color((0x00, 0x7a, 0xcc))
}