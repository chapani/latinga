use crate::BARCHA_TUTUQ_TURLARI;
use crate::engine::ChunkProcessor;
use std::borrow::Cow;

/// Port for handling specific Latin-to-Latin mappings (e.g., Kelgusi mode).
pub trait LatinMapper {
    fn handle_latin_to_latin(
        &self,
        index: usize,
        c: char,
        char_len: usize,
        prev: Option<char>,
    ) -> (Cow<'static, str>, usize);
}

impl<'a> LatinMapper for ChunkProcessor<'a> {
    fn handle_latin_to_latin(
        &self,
        index: usize,
        c: char,
        char_len: usize,
        prev: Option<char>,
    ) -> (Cow<'static, str>, usize) {
        let next_idx = index + char_len;
        let next_char = self.peek_char(next_idx);

        if next_char.is_none() {
            return (Cow::Owned(c.to_string()), char_len);
        }

        let next = next_char.unwrap();
        let next_lower = next.to_ascii_lowercase();
        let current_lower = c.to_ascii_lowercase();

        let caps = self.is_caps_context(index, char_len, prev);

        match (current_lower, next_lower) {
            ('s', 'h') => (
                self.format_complex("ş", c.is_uppercase(), caps),
                char_len + next.len_utf8(),
            ),
            ('c', 'h') => (
                self.format_complex("ç", c.is_uppercase(), caps),
                char_len + next.len_utf8(),
            ),
            ('o', m) if BARCHA_TUTUQ_TURLARI.contains(&m) => (
                self.format_complex("ö", c.is_uppercase(), caps),
                char_len + next.len_utf8(),
            ),
            ('g', m) if BARCHA_TUTUQ_TURLARI.contains(&m) => (
                self.format_complex("ğ", c.is_uppercase(), caps),
                char_len + next.len_utf8(),
            ),
            _ => (Cow::Owned(c.to_string()), char_len),
        }
    }
}
