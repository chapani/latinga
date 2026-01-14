use crate::engine::ChunkProcessor;
use crate::{TUTUQ, Tartib};
use std::borrow::Cow;

/// Port for handling specific Cyrillic character mappings.
pub trait CyrillicMapper {
    fn handle_ye(
        &self,
        c: char,
        prev: Option<char>,
        index: usize,
        char_len: usize,
    ) -> Cow<'static, str>;

    fn handle_ts(
        &self,
        c: char,
        prev: Option<char>,
        index: usize,
        char_len: usize,
    ) -> Cow<'static, str>;

    fn handle_hard_sign(
        &self,
        index: usize,
        char_len: usize,
        prev: Option<char>,
    ) -> (Cow<'static, str>, usize);
}

impl<'a> CyrillicMapper for ChunkProcessor<'a> {
    fn handle_ye(
        &self,
        c: char,
        prev: Option<char>,
        index: usize,
        char_len: usize,
    ) -> Cow<'static, str> {
        let is_upper = c.is_uppercase();
        let prev_was_hard = prev.map_or(false, |p| p == 'Ъ' || p == 'ъ');

        if prev_was_hard {
            if is_upper {
                Cow::Borrowed("E")
            } else {
                Cow::Borrowed("e")
            }
        } else if (index == 0 && self.prev_char_boundary.is_none())
            || prev.map_or(true, |p| !p.is_alphabetic())
            || self.is_vowel(prev)
        {
            self.format_complex("Ye", is_upper, self.is_caps_context(index, char_len, prev))
        } else if is_upper {
            Cow::Borrowed("E")
        } else {
            Cow::Borrowed("e")
        }
    }

    fn handle_ts(
        &self,
        c: char,
        prev: Option<char>,
        index: usize,
        char_len: usize,
    ) -> Cow<'static, str> {
        let is_upper = c.is_uppercase();
        if self.is_vowel(prev) {
            self.format_complex("Ts", is_upper, self.is_caps_context(index, char_len, prev))
        } else if is_upper {
            Cow::Borrowed("S")
        } else {
            Cow::Borrowed("s")
        }
    }

    fn handle_hard_sign(
        &self,
        index: usize,
        char_len: usize,
        prev: Option<char>,
    ) -> (Cow<'static, str>, usize) {
        let next_idx = index + char_len;
        let next = self.peek_char(next_idx).map(|c| c.to_ascii_lowercase());
        let prev_lower = prev.map(|c| c.to_ascii_lowercase());

        match (prev_lower, next) {
            (_, Some('е' | 'ю' | 'я'))
                if prev_lower.is_some_and(|p| "бвгджзйклмнпрстфхцчшщқҳ".contains(p)) =>
            {
                (Cow::Borrowed("y"), char_len)
            }
            (_, Some('е')) => (Cow::Borrowed(""), char_len),
            _ => {
                if self.config.tartib == Tartib::Kelgusi {
                    (Cow::Borrowed(""), char_len)
                } else {
                    (Cow::Owned(TUTUQ.to_string()), char_len)
                }
            }
        }
    }
}
