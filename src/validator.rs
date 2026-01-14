use crate::shield::Shield;
use crate::symbols::KELGUSI_MAP;
use crate::types::Chunk;
use crate::{
    BARCHA_TUTUQ_TURLARI, OKINA, Sozlama, TUTUQ, Tartib, TekshiruvHatosi, TekshiruvHulosasi,
};
use std::borrow::Cow;

/// Validator holds a reference to the configuration.
/// Lifetime 'a refers to the lifespan of the Config struct.
pub struct Validator<'a> {
    config: &'a Sozlama,
}

impl<'a> Validator<'a> {
    pub fn new(config: &'a Sozlama) -> Self {
        Self { config }
    }

    /// Scans the input for errors.
    /// Introduces a new lifetime 'b for the input string, decoupling it from Config.
    pub fn check_errors<'b>(&self, input: &'b str, limit: usize) -> TekshiruvHulosasi<'b> {
        // Pre-allocate a reasonable amount to avoid re-allocations
        let mut errors = Vec::with_capacity(limit.min(100));
        let mut total_count = 0;
        let mut line = 1;
        let mut col = 1;

        // Shield::tokenize will return chunks referencing 'b (input)
        let chunks = Shield::tokenize(input, self.config);

        for chunk in chunks {
            match chunk {
                Chunk::Shielded(text) => {
                    // Fast scan for newlines in shielded blocks
                    for byte in text.bytes() {
                        if byte == b'\n' {
                            line += 1;
                            col = 1;
                        } else if (byte & 0xC0) != 0x80 {
                            col += 1;
                        }
                    }
                }
                Chunk::Safe(text) => {
                    self.process_safe_chunk(
                        text,
                        &mut line,
                        &mut col,
                        &mut errors,
                        &mut total_count,
                        limit,
                    );
                }
            }
        }

        TekshiruvHulosasi {
            hatolar: errors,
            jami: total_count,
        }
    }

    // This helper now explicitly uses 'b for the input text/errors
    fn process_safe_chunk<'b>(
        &self,
        text: &'b str,
        line: &mut usize,
        col: &mut usize,
        errors: &mut Vec<TekshiruvHatosi<'b>>,
        total_count: &mut usize,
        limit: usize,
    ) {
        let mut word_start: Option<(usize, usize)> = None;
        let mut prev_char: Option<char> = None;

        for (byte_idx, c) in text.char_indices() {
            // 1. Handle Newlines
            if c == '\n' {
                if let Some((start_idx, start_col)) = word_start {
                    let word = &text[start_idx..byte_idx];
                    self.check_word(word, *line, start_col, errors, total_count, limit);
                    word_start = None;
                }
                *line += 1;
                *col = 1;
                prev_char = Some(c);
                continue;
            }

            // 2. Identify Word Characters
            let is_word_char = c.is_alphabetic() || BARCHA_TUTUQ_TURLARI.contains(&c);

            if is_word_char {
                if word_start.is_none() {
                    word_start = Some((byte_idx, *col));
                }

                if self.config.tartib == Tartib::Joriy && BARCHA_TUTUQ_TURLARI.contains(&c) {
                    if let Some(msg) = self.check_apostrophe_inline(c, prev_char) {
                        *total_count += 1;
                        if errors.len() < limit {
                            let start_idx = word_start.unwrap().0;
                            let end_idx = self.find_word_end(text, byte_idx);
                            let full_word = &text[start_idx..end_idx];

                            errors.push(TekshiruvHatosi {
                                qator: *line,
                                ustun: *col,
                                soez: Cow::Borrowed(full_word),
                                habar: msg,
                            });
                        }
                    }
                }
            } else {
                if let Some((start_idx, start_col)) = word_start {
                    let word = &text[start_idx..byte_idx];
                    self.check_word(word, *line, start_col, errors, total_count, limit);
                    word_start = None;
                }
            }

            *col += 1;
            prev_char = Some(c);
        }

        if let Some((start_idx, start_col)) = word_start {
            let word = &text[start_idx..];
            self.check_word(word, *line, start_col, errors, total_count, limit);
        }
    }

    fn find_word_end(&self, text: &str, start_search: usize) -> usize {
        text[start_search..]
            .find(|c: char| !c.is_alphabetic() && !BARCHA_TUTUQ_TURLARI.contains(&c))
            .map(|offset| start_search + offset)
            .unwrap_or(text.len())
    }

    #[inline(always)]
    fn check_apostrophe_inline(&self, c: char, prev: Option<char>) -> Option<Cow<'static, str>> {
        if c == OKINA || c == TUTUQ {
            return None;
        }

        if matches!(c, '\'' | '`' | '‘' | '’') {
            if let Some(p) = prev {
                let pl = p.to_ascii_lowercase();
                if pl == 'o' || pl == 'g' {
                    return Some(
                        crate::HabarKaliti::EskiQoeshHarf(OKINA.to_string())
                            .koersat(&self.config.tartib),
                    );
                }
            }
            return Some(
                crate::HabarKaliti::EskiQoeshHarf(TUTUQ.to_string()).koersat(&self.config.tartib),
            );
        }
        None
    }

    fn check_word<'b>(
        &self,
        word: &'b str,
        line: usize,
        col: usize,
        errors: &mut Vec<TekshiruvHatosi<'b>>,
        total_count: &mut usize,
        limit: usize,
    ) {
        if word.is_empty() {
            return;
        }

        let is_counting_only = errors.len() >= limit;

        match self.config.tartib {
            Tartib::Joriy => {
                let has_s = word.contains(['s', 'S']);
                let has_h = word.contains(['h', 'H']);

                if has_s && has_h {
                    let wl = word.to_lowercase();
                    if let Some(mat) = crate::engine::RE_SH.find(&wl) {
                        if !word.chars().any(|c| BARCHA_TUTUQ_TURLARI.contains(&c)) {
                            *total_count += 1;
                            if !is_counting_only {
                                let char_offset = word[..mat.start()].chars().count();
                                errors.push(TekshiruvHatosi {
                                    qator: line,
                                    ustun: col + char_offset,
                                    soez: Cow::Borrowed(word),
                                    habar: crate::HabarKaliti::QoeshHarfTutuqlaAjratilsin
                                        .koersat(&self.config.tartib),
                                });
                            }
                        }
                    }
                }
            }
            Tartib::Kelgusi => {
                for (trigger, replacement) in KELGUSI_MAP {
                    if let Some(byte_pos) = self.find_case_insensitive(word, trigger) {
                        *total_count += 1;
                        if !is_counting_only {
                            let is_upper = word.chars().next().is_some_and(|c| c.is_uppercase());
                            let char_offset = word[..byte_pos].chars().count();
                            let s = if is_upper {
                                replacement.to_uppercase()
                            } else {
                                replacement.to_lowercase()
                            };

                            errors.push(TekshiruvHatosi {
                                qator: line,
                                ustun: col + char_offset,
                                soez: Cow::Borrowed(word),
                                habar: crate::HabarKaliti::EskiQoeshHarf(s)
                                    .koersat(&self.config.tartib),
                            });
                        }
                        return;
                    }
                }
            }
        }
    }

    fn find_case_insensitive(&self, haystack: &str, needle: &str) -> Option<usize> {
        if haystack.len() < needle.len() {
            return None;
        }

        let needle_bytes = needle.as_bytes();

        for (i, _) in haystack.char_indices() {
            let slice = &haystack[i..];
            if slice.len() < needle.len() {
                break;
            }

            let mut matched = true;
            for (j, b) in needle_bytes.iter().enumerate() {
                let h_char = slice.as_bytes()[j] as char;
                if !h_char.eq_ignore_ascii_case(&(*b as char)) {
                    matched = false;
                    break;
                }
            }
            if matched {
                return Some(i);
            }
        }
        None
    }
}
