pub(crate) mod cyrillic;
pub(crate) mod latin;

// Re-export traits for internal usage
pub(crate) use cyrillic::CyrillicMapper;
pub(crate) use latin::LatinMapper;

use crate::shield::Shield;
use crate::types::Chunk;
use crate::{
    Sozlama, Tartib,
    symbols::{BARCHA_TUTUQ_TURLARI, CYR_VOWELS, MAP_1_TO_1, OKINA, OKINA_STR, TUTUQ, TUTUQ_STR},
};
use regex::Regex;
use std::borrow::Cow;
use std::io::{self, Write};
use std::sync::LazyLock;

// Optimization: Pre-compile regex for 'sh'/'ch' collision detection in Joriy mode
pub(crate) static RE_SH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(is|as|mus)(hoq|hob|haf)$").unwrap());

/// The main entry point for the transliteration logic.
/// Stateless wrapper around configuration.
pub struct Engine<'a> {
    pub(crate) config: &'a Sozlama,
}

impl<'a> Engine<'a> {
    #[must_use]
    pub fn new(config: &'a Sozlama) -> Self {
        Self { config }
    }

    /// Convenience method for string-to-string conversion.
    /// Allocates a buffer internally.
    pub fn run(&self, input: &str) -> String {
        let mut buf = Vec::with_capacity(input.len());
        self.convert_stream(input, &mut buf)
            .expect("In-memory buffer write failed");
        String::from_utf8(buf).unwrap_or_else(|_| input.to_string())
    }

    /// Core streaming conversion method.
    /// Uses a reusable buffer to minimize allocation during word processing.
    pub fn convert_stream<W: Write + ?Sized>(&self, input: &str, writer: &mut W) -> io::Result<()> {
        let mut chunks = Shield::tokenize(input, self.config).peekable();
        let mut prev_char: Option<char> = None;

        // OPTIMIZATION: Reusable buffer for words.
        // Allocated ONCE, reused for every word in the stream.
        let mut word_buffer = String::with_capacity(64);

        while let Some(chunk) = chunks.next() {
            match chunk {
                Chunk::Shielded(text) => {
                    writer.write_all(text.as_bytes())?;
                    prev_char = text.chars().last();
                }
                Chunk::Safe(text) => {
                    let next_is_shielded = matches!(chunks.peek(), Some(Chunk::Shielded(_)));

                    let mut processor =
                        ChunkProcessor::new(text, prev_char, next_is_shielded, self.config);

                    // Pass the reused buffer to avoid inner loop allocations
                    processor.process_to_writer(writer, &mut word_buffer)?;

                    prev_char = text.chars().last();
                }
            }
        }
        Ok(())
    }
}

/// Context-aware processor for a specific "Safe" text chunk.
/// Maintains state (previous character, boundaries) for the duration of the chunk.
pub(crate) struct ChunkProcessor<'a> {
    text: &'a str,
    prev_char_boundary: Option<char>,
    next_is_shielded: bool,
    config: &'a Sozlama,
}

impl<'a> ChunkProcessor<'a> {
    fn new(text: &'a str, prev: Option<char>, next_shielded: bool, config: &'a Sozlama) -> Self {
        Self {
            text,
            prev_char_boundary: prev,
            next_is_shielded: next_shielded,
            config,
        }
    }

    fn process_to_writer<W: Write + ?Sized>(
        &mut self,
        writer: &mut W,
        word_buffer: &mut String,
    ) -> io::Result<()> {
        let mut byte_idx = 0;
        let mut prev_char = self.prev_char_boundary;

        while byte_idx < self.text.len() {
            let rest = &self.text[byte_idx..];
            let c = rest.chars().next().unwrap();
            let char_len = c.len_utf8();

            let is_boundary = prev_char.map_or(true, |p| !p.is_alphabetic());

            // 1. Trie Lookup (Healing / Exceptions / Proper Nouns)
            // Only performed at word boundaries for performance
            if is_boundary && c.is_alphabetic() {
                if let Some((match_bytes, replacement)) = self.find_trie_match(byte_idx) {
                    // Flush pending word buffer before writing direct replacement
                    if !word_buffer.is_empty() {
                        self.flush_word(writer, word_buffer)?;
                    }

                    let original = &self.text[byte_idx..byte_idx + match_bytes];
                    let cased_replacement = self.apply_casing(&replacement, original);
                    writer.write_all(cased_replacement.as_bytes())?;

                    prev_char = original.chars().last();
                    byte_idx += match_bytes;
                    continue;
                }
            }

            // 2. Character Conversion
            let (converted, consumed_bytes) = self.convert_char(byte_idx, c, prev_char);

            // 3. Buffer Management
            // Connecting hyphens (e.g., "tarbiyaviy-axloqiy") are treated as part of the word
            // to allow suffix analysis in Kelgusi mode.
            let is_connecting_hyphen = c == '-'
                && prev_char.map_or(false, |p| p.is_alphabetic())
                && self
                    .peek_char(byte_idx + char_len)
                    .map_or(false, |n| n.is_alphabetic());

            if c.is_alphabetic() || is_connecting_hyphen {
                word_buffer.push_str(&converted);
            } else {
                if !word_buffer.is_empty() {
                    self.flush_word(writer, word_buffer)?;
                }
                writer.write_all(converted.as_bytes())?;
            }

            let consumed_str = &self.text[byte_idx..byte_idx + consumed_bytes];
            prev_char = consumed_str.chars().last();
            byte_idx += consumed_bytes;
        }

        if !word_buffer.is_empty() {
            self.flush_word(writer, word_buffer)?;
        }
        Ok(())
    }

    /// Flushes the accumulated word buffer to the writer.
    /// Handles post-processing logic:
    /// - Joriy: Apostrophe standardization and 'is'hoq' collision fixes.
    /// - Kelgusi: Proper noun suffix separation (e.g., "Toshkent'da").
    fn flush_word<W: Write + ?Sized>(
        &self,
        writer: &mut W,
        word_buffer: &mut String,
    ) -> io::Result<()> {
        if word_buffer.is_empty() {
            return Ok(());
        }

        match self.config.tartib {
            Tartib::Joriy => {
                let needs_cleaning = word_buffer.contains(OKINA) || word_buffer.contains(TUTUQ);
                let text_cow = if needs_cleaning {
                    Cow::Owned(
                        word_buffer
                            .replace(&format!("{}{}", OKINA, TUTUQ), &OKINA.to_string())
                            .replace(&format!("{}{}", TUTUQ, TUTUQ), &TUTUQ.to_string())
                            .replace(&format!("{}{}", OKINA, OKINA), &OKINA.to_string()),
                    )
                } else {
                    Cow::Borrowed(word_buffer.as_str())
                };

                let text_to_check = text_cow.as_ref();
                if let Some(caps) = RE_SH.captures(text_to_check) {
                    writer.write_all(caps[1].as_bytes())?;
                    writer.write_all(TUTUQ_STR.as_bytes())?;
                    writer.write_all(caps[2].as_bytes())?;
                } else {
                    writer.write_all(text_to_check.as_bytes())?;
                }
            }
            Tartib::Kelgusi => {
                let mut found_suffix = false;

                if let Some(first_char) = word_buffer.chars().next() {
                    let c_lower = first_char.to_ascii_lowercase();
                    // Fast filter using boolean array
                    let should_check = (c_lower as u32) >= 128
                        || self.config.lughat.proper_noun_first_chars[c_lower as usize];

                    if should_check {
                        if let Some((byte_len, stored_proper_noun)) = self
                            .config
                            .lughat
                            .proper_nouns_trie
                            .find_longest_prefix(word_buffer)
                        {
                            if byte_len < word_buffer.len() {
                                let stem = &word_buffer[..byte_len];
                                if Self::is_valid_casing(stem, stored_proper_noun) {
                                    let suffix = &word_buffer[byte_len..];
                                    if self.config.lughat.suffixes.contains(&suffix.to_lowercase())
                                    {
                                        writer.write_all(stem.as_bytes())?;
                                        writer.write_all(b"'")?;
                                        writer.write_all(suffix.as_bytes())?;
                                        found_suffix = true;
                                    }
                                }
                            }
                        }
                    }
                }

                if !found_suffix {
                    writer.write_all(word_buffer.as_bytes())?;
                }
            }
        }
        word_buffer.clear();
        Ok(())
    }

    fn convert_char(
        &self,
        index: usize,
        c: char,
        prev: Option<char>,
    ) -> (Cow<'static, str>, usize) {
        let is_kelgusi = self.config.tartib == Tartib::Kelgusi;
        let char_len = c.len_utf8();

        match c {
            c if BARCHA_TUTUQ_TURLARI.contains(&c) => {
                if let Some(p) = prev {
                    let prev_lower = p.to_ascii_lowercase();

                    if (prev_lower == 'o' || prev_lower == 'g') && p.is_alphabetic() {
                        return (Cow::Borrowed(OKINA_STR), char_len);
                    }

                    if is_kelgusi && p.is_alphabetic() {
                        if let Some(next) = self.peek_char(index + char_len) {
                            if next.is_alphabetic() {
                                return (Cow::Borrowed(""), char_len);
                            }
                        }
                    }

                    if p.is_alphabetic() {
                        if let Some(next) = self.peek_char(index + char_len) {
                            if next.is_alphabetic() {
                                return (Cow::Borrowed(TUTUQ_STR), char_len);
                            }
                        }
                    }
                }
                (Cow::Owned(c.to_string()), char_len)
            }
            '\u{0312}' | '\u{0300}' | '\u{0301}' | '\u{0306}' => {
                if let Some(p) = prev {
                    let prev_lower = p.to_ascii_lowercase();
                    if prev_lower == 'o' || prev_lower == 'g' {
                        return (Cow::Borrowed(OKINA_STR), char_len);
                    }
                }
                (Cow::Borrowed(""), char_len)
            }
            // Cyrillic Logic (Delegated to Trait)
            'Е' | 'е' => (self.handle_ye(c, prev, index, char_len), char_len),
            'Ц' | 'ц' => (self.handle_ts(c, prev, index, char_len), char_len),
            'Ъ' | 'ъ' => self.handle_hard_sign(index, char_len, prev),
            'Ь' | 'ь' => (Cow::Borrowed(""), char_len),
            'Ё' | 'ё' => (
                self.format_complex(
                    "Yo",
                    c.is_uppercase(),
                    self.is_caps_context(index, char_len, prev),
                ),
                char_len,
            ),
            'Ю' | 'ю' => (
                self.format_complex(
                    "Yu",
                    c.is_uppercase(),
                    self.is_caps_context(index, char_len, prev),
                ),
                char_len,
            ),
            'Я' | 'я' => (
                self.format_complex(
                    "Ya",
                    c.is_uppercase(),
                    self.is_caps_context(index, char_len, prev),
                ),
                char_len,
            ),
            'Ғ' | 'ғ' => {
                let rep = if is_kelgusi {
                    "ğ"
                } else {
                    return (
                        Cow::Owned(
                            self.format_complex(
                                &format!("G{OKINA}"),
                                c.is_uppercase(),
                                self.is_caps_context(index, char_len, prev),
                            )
                            .into_owned(),
                        ),
                        char_len,
                    );
                };
                (
                    self.format_complex(
                        rep,
                        c.is_uppercase(),
                        self.is_caps_context(index, char_len, prev),
                    ),
                    char_len,
                )
            }
            'Ў' | 'ў' => {
                let rep = if is_kelgusi {
                    "ö"
                } else {
                    return (
                        Cow::Owned(
                            self.format_complex(
                                &format!("o{OKINA}"),
                                c.is_uppercase(),
                                self.is_caps_context(index, char_len, prev),
                            )
                            .into_owned(),
                        ),
                        char_len,
                    );
                };
                (
                    self.format_complex(
                        rep,
                        c.is_uppercase(),
                        self.is_caps_context(index, char_len, prev),
                    ),
                    char_len,
                )
            }
            'Ш' | 'ш' | 'Щ' | 'щ' => (
                self.format_complex(
                    if is_kelgusi { "ş" } else { "sh" },
                    c.is_uppercase(),
                    self.is_caps_context(index, char_len, prev),
                ),
                char_len,
            ),
            'Ч' | 'ч' => (
                self.format_complex(
                    if is_kelgusi { "ç" } else { "ch" },
                    c.is_uppercase(),
                    self.is_caps_context(index, char_len, prev),
                ),
                char_len,
            ),
            // Latin Logic (Delegated to Trait)
            's' | 'S' | 'c' | 'C' | 'o' | 'O' | 'g' | 'G' if is_kelgusi => {
                self.handle_latin_to_latin(index, c, char_len, prev)
            }
            _ => (self.handle_default(c), char_len),
        }
    }

    fn find_trie_match(&self, i: usize) -> Option<(usize, String)> {
        let rest = &self.text[i..];
        let c = rest.chars().next()?;
        let c_lower_char = c.to_ascii_lowercase();

        // 1. Healing
        let should_check_healing = (c_lower_char as u32) >= 128
            || self.config.lughat.healing_first_chars[c_lower_char as usize];

        if should_check_healing {
            if let Some((len, replacement)) =
                self.config.lughat.healing_trie.find_longest_prefix(rest)
            {
                let is_proper_noun = self
                    .config
                    .lughat
                    .proper_nouns_trie
                    .find_longest_prefix(&rest[..len])
                    .map_or(false, |(pn_len, _)| pn_len == len);
                if !is_proper_noun {
                    return Some((len, replacement.to_string()));
                }
            }
        }

        // 2. Exceptions
        if let Some((len, replacement)) = self
            .config
            .lughat
            .substitutions_trie
            .find_longest_prefix(rest)
        {
            return Some((len, replacement.to_string()));
        }

        // 3. Proper Nouns
        let should_check_proper = (c_lower_char as u32) >= 128
            || self.config.lughat.proper_noun_first_chars[c_lower_char as usize];

        if should_check_proper {
            if let Some((stem_len, stored_proper_noun)) = self
                .config
                .lughat
                .proper_nouns_trie
                .find_longest_prefix(rest)
            {
                let input_stem = &rest[..stem_len];

                if !Self::is_valid_casing(input_stem, stored_proper_noun) {
                    return None;
                }

                let tail_slice = &rest[stem_len..];
                let suffix_len = tail_slice
                    .find(|sc: char| !sc.is_alphabetic())
                    .unwrap_or(tail_slice.len());
                let raw_suffix = &tail_slice[..suffix_len];

                let suffix_valid = raw_suffix.is_empty()
                    || self
                        .config
                        .lughat
                        .suffixes
                        .contains(&raw_suffix.to_lowercase());

                if suffix_valid {
                    let mut res = input_stem.to_string();
                    if !raw_suffix.is_empty() {
                        if self.config.tartib == Tartib::Kelgusi {
                            res.push('\'');
                        }
                        res.push_str(raw_suffix);
                    }
                    return Some((stem_len + suffix_len, res));
                }
            }
        }
        None
    }

    fn handle_default(&self, c: char) -> Cow<'static, str> {
        let is_kelgusi = self.config.tartib == Tartib::Kelgusi;
        if c == 'Х' || c == 'х' || (is_kelgusi && (c == 'X' || c == 'x')) {
            let h = if c.is_uppercase() { "H" } else { "h" };
            let x = if c.is_uppercase() { "X" } else { "x" };
            return if is_kelgusi {
                Cow::Borrowed(h)
            } else {
                Cow::Borrowed(x)
            };
        }
        if let Some((_, lat)) = MAP_1_TO_1.iter().find(|(cyr, _)| *cyr == c) {
            return Cow::Borrowed(*lat);
        }
        Cow::Owned(c.to_string())
    }

    // --- Helpers used by Submodules (Cyrillic/Latin) ---

    pub(crate) fn peek_char(&self, index: usize) -> Option<char> {
        if index >= self.text.len() {
            return None;
        }
        self.text[index..].chars().next()
    }

    pub(crate) fn is_vowel(&self, c: Option<char>) -> bool {
        if let Some(ch) = c {
            CYR_VOWELS.contains(ch)
        } else {
            false
        }
    }

    pub(crate) fn is_caps_context(&self, i: usize, c_len: usize, prev: Option<char>) -> bool {
        let prev_caps = prev.map_or(false, char::is_uppercase);
        let next_caps = self.peek_char(i + c_len).map_or(false, |c| {
            let is_end = i + c_len + c.len_utf8() >= self.text.len();
            if is_end && self.next_is_shielded {
                false
            } else {
                c.is_uppercase()
            }
        });
        prev_caps || next_caps
    }

    pub(crate) fn apply_casing(&self, replacement: &str, original: &str) -> Cow<'static, str> {
        let first_char = original.chars().next().unwrap();
        let is_upper = first_char.is_uppercase();
        let is_all_caps = original.len() > 1 && original.chars().all(|c| !c.is_lowercase());

        if is_all_caps {
            Cow::Owned(replacement.to_uppercase())
        } else if is_upper {
            let mut c = replacement.chars();
            let res = c.next().map_or(String::new(), |f| {
                f.to_uppercase().collect::<String>() + c.as_str()
            });
            Cow::Owned(res)
        } else {
            Cow::Owned(replacement.to_string())
        }
    }

    pub(crate) fn format_complex(
        &self,
        rep: &str,
        is_upper: bool,
        caps_context: bool,
    ) -> Cow<'static, str> {
        if !is_upper {
            return Cow::Owned(rep.to_lowercase());
        }
        if caps_context {
            Cow::Owned(rep.to_uppercase())
        } else {
            let mut c = rep.chars();
            Cow::Owned(c.next().unwrap().to_uppercase().collect::<String>() + c.as_str())
        }
    }

    fn is_valid_casing(input_stem: &str, stored_value: &str) -> bool {
        if input_stem == stored_value {
            return true;
        }
        let first_stored = stored_value.chars().next().unwrap_or_default();
        let input_is_caps = input_stem.chars().all(|c| !c.is_lowercase());

        if first_stored.is_uppercase() {
            if input_is_caps {
                return true;
            }
        } else {
            if input_is_caps && input_stem.to_lowercase() == stored_value.to_lowercase() {
                return true;
            }
            let mut input_chars = input_stem.chars();
            let mut stored_chars = stored_value.chars();

            if let (Some(ic), Some(_)) = (input_chars.next(), stored_chars.next()) {
                if ic.is_uppercase() {
                    if input_chars.as_str() == stored_chars.as_str() {
                        return true;
                    }
                }
            }
        }
        false
    }
}
