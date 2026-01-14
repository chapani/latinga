use crate::types::Chunk;
use crate::{BARCHA_TUTUQ_TURLARI, Sozlama};
use regex::Regex;
use std::collections::VecDeque;
use std::sync::LazyLock;

// --- 1. TOKEN-LEVEL REGEXES ---
static RE_ROMAN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\bM{0,4}(CM|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})\b").unwrap()
});
static RE_CODE_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?ms)```.*?```|`[^`]+`").unwrap());
static RE_EMAIL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap());
static RE_URL: LazyLock<Regex> = LazyLock::new(|| {
    let apostrophes: String = BARCHA_TUTUQ_TURLARI.iter().collect();
    let pattern = format!(r#"(?i)\bhttps?://[^\s<>"{}]+"#, regex::escape(&apostrophes));
    Regex::new(&pattern).unwrap()
});
static RE_HTML_ENTITY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"&[a-zA-Z0-9#]+;").unwrap());
static RE_KEY_VALUE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[a-zA-Z0-9_-]+\s*=\s*[a-zA-Z0-9_\\\-]+").unwrap());
static RE_ATTR_SCAN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)([a-z0-9\-]+)\s*=\s*(["'][^"']*["'])"#).unwrap());

const LATEX_STRUCTURAL_COMMANDS: &[&str] = &[
    "label",
    "cite",
    "ref",
    "include",
    "input",
    "includegraphics",
    "usepackage",
    "documentclass",
    "begin",
    "end",
];
const LATEX_VERBATIM_ENVIRONMENTS: &[&str] = &["verbatim", "lstlisting", "code", "minted"];

// --- 2. THE PUBLIC API ---

pub struct Shield;

impl Shield {
    pub fn tokenize<'a>(input: &'a str, config: &Sozlama) -> TokenIterator<'a> {
        TokenIterator::new(input, config)
    }
}

pub(crate) struct TokenIterator<'a> {
    input: &'a str,
    cursor: usize,
    mask: Vec<bool>,
    buffer: VecDeque<Chunk<'a>>,
}

impl<'a> TokenIterator<'a> {
    fn new(input: &'a str, config: &Sozlama) -> Self {
        let mut mask = vec![false; input.len()];
        Self::build_pre_mask(input, config, &mut mask);

        Self {
            input,
            cursor: 0,
            mask,
            buffer: VecDeque::new(),
        }
    }

    fn build_pre_mask(input: &str, config: &Sozlama, mask: &mut [bool]) {
        let token_regexes = [
            &*RE_CODE_BLOCK,
            &*RE_URL,
            &*RE_EMAIL,
            &*RE_ROMAN,
            &*RE_HTML_ENTITY,
            &*RE_KEY_VALUE,
        ];

        for re in token_regexes {
            for m in re.find_iter(input) {
                if let Some(slice) = mask.get_mut(m.start()..m.end()) {
                    slice.fill(true);
                }
            }
        }

        for re in &config.lughat.qalqons_re {
            for cap in re.captures_iter(input) {
                let range = if let Some(inner) = cap.get(1) {
                    inner.range()
                } else {
                    cap.get(0).unwrap().range()
                };
                if let Some(slice) = mask.get_mut(range) {
                    slice.fill(true);
                }
            }
        }

        if let Some(ref ac) = config.lughat.qalqons_ac {
            for mat in ac.find_iter(input) {
                let start = mat.start();
                let end = mat.end();
                let valid_start = start == 0 || !is_word_char(input, start - 1);
                let valid_end = end == input.len() || !is_word_char(input, end);

                if valid_start && valid_end {
                    if let Some(slice) = mask.get_mut(start..end) {
                        slice.fill(true);
                    }
                }
            }
        }
    }

    // --- CORE ITERATION LOGIC ---

    fn scan_next(&mut self) -> Option<Chunk<'a>> {
        let len = self.input.len();
        if self.cursor >= len {
            return None;
        }

        let start = self.cursor;

        // 1. Check Pre-Mask
        if self.mask[self.cursor] {
            while self.cursor < len && self.mask[self.cursor] {
                self.cursor += 1;
            }
            return Some(Chunk::Shielded(&self.input[start..self.cursor]));
        }

        let c = self.peek(0)?;

        // Universal Shield {] ... [}
        if c == '{' && self.peek(1) == Some(']') {
            if let Some(end_idx) = self.scan_universal_shield() {
                // inner_start skips the opening "{]"
                let inner_start = start + 2;
                // inner_end stops before the closing "[}"
                // end_idx is the position AFTER the closing "}", so we subtract 2
                let inner_end = end_idx.saturating_sub(2).max(inner_start);

                self.cursor = end_idx;

                if inner_start >= inner_end {
                    // Empty shield case {][}, just recurse
                    return self.scan_next();
                }
                return Some(Chunk::Shielded(&self.input[inner_start..inner_end]));
            }
        }

        // LaTeX Shield
        if matches!(c, '\\' | '%' | '$') {
            if let Some(end_idx) = self.scan_latex() {
                let chunk = Chunk::Shielded(&self.input[start..end_idx]);
                self.cursor = end_idx;
                return Some(chunk);
            }
        }

        // HTML Smart Shield
        if c == '<' {
            if let Some((end_idx, is_opaque)) = self.scan_html() {
                if is_opaque {
                    let chunk = Chunk::Shielded(&self.input[start..end_idx]);
                    self.cursor = end_idx;
                    return Some(chunk);
                } else {
                    self.emit_smart_tag(start, end_idx);
                    self.cursor = end_idx;
                    return self.buffer.pop_front();
                }
            }
        }

        // 3. Neutral State (Consume 'Safe' chars)
        let bytes = self.input.as_bytes();
        while self.cursor < len {
            if self.mask[self.cursor] {
                break;
            }
            let curr = bytes[self.cursor] as char;
            if matches!(curr, '\\' | '%' | '$' | '<') {
                break;
            }
            if curr == '{' && self.peek(1) == Some(']') {
                break;
            }
            self.cursor += 1;
        }

        if self.cursor > start {
            Some(Chunk::Safe(&self.input[start..self.cursor]))
        } else {
            if self.cursor < len {
                self.cursor += 1;
                Some(Chunk::Safe(&self.input[start..self.cursor]))
            } else {
                None
            }
        }
    }

    fn emit_smart_tag(&mut self, start: usize, end: usize) {
        let tag_content = &self.input[start..end];
        let mut last_idx = 0;

        for cap in RE_ATTR_SCAN.captures_iter(tag_content) {
            let name = cap.get(1).unwrap();
            let val_match = cap.get(2).unwrap();
            let attr_name = name.as_str().to_lowercase();

            if crate::symbols::TRANSLITERABLE_ATTRIBUTES.contains(&attr_name.as_str()) {
                let full_val_str = val_match.as_str();

                let (quote_len, _) = if full_val_str.starts_with(['"', '\'']) {
                    (1, 1)
                } else {
                    (0, 0)
                };

                let abs_val_start = start + val_match.start() + quote_len;
                let abs_val_end = start + val_match.end() - quote_len;
                let relative_val_start = val_match.start() + quote_len;

                if relative_val_start > last_idx {
                    self.buffer
                        .push_back(Chunk::Shielded(&tag_content[last_idx..relative_val_start]));
                }

                let mut v_curr = abs_val_start;
                while v_curr < abs_val_end {
                    let v_chunk_start = v_curr;
                    if self.mask[v_curr] {
                        while v_curr < abs_val_end && self.mask[v_curr] {
                            v_curr += 1;
                        }
                        self.buffer
                            .push_back(Chunk::Shielded(&self.input[v_chunk_start..v_curr]));
                    } else {
                        while v_curr < abs_val_end && !self.mask[v_curr] {
                            v_curr += 1;
                        }
                        self.buffer
                            .push_back(Chunk::Safe(&self.input[v_chunk_start..v_curr]));
                    }
                }
                last_idx = val_match.end() - quote_len;
            }
        }

        if last_idx < tag_content.len() {
            self.buffer
                .push_back(Chunk::Shielded(&tag_content[last_idx..]));
        }
    }

    // --- HELPER SCANNERS ---

    fn peek(&self, offset: usize) -> Option<char> {
        self.input
            .as_bytes()
            .get(self.cursor + offset)
            .map(|&b| b as char)
    }

    fn scan_universal_shield(&self) -> Option<usize> {
        // Find closing "[}"
        // The return value must be the index AFTER the closing brace
        self.input[self.cursor..]
            .find("[}")
            .map(|idx| self.cursor + idx + 2)
    }

    fn scan_html(&self) -> Option<(usize, bool)> {
        let rest = &self.input[self.cursor..];
        let tag_end = rest.find(|c: char| c.is_whitespace() || c == '>' || c == '/')?;

        let tag_name = &rest[1..tag_end];
        let tag_lower = tag_name.to_lowercase();

        // 1. Fully Protected Tags (script, style, etc.)
        if crate::symbols::FULLY_PROTECTED_TAGS.contains(&tag_lower.as_str()) {
            let close_target = format!("</{}>", tag_lower);
            if let Some(close_pos) = find_case_insensitive(rest, &close_target) {
                return Some((self.cursor + close_pos + close_target.len(), true));
            }
        }

        // 2. Standard Tag
        if let Some(end_idx) = rest.find('>') {
            return Some((self.cursor + end_idx + 1, false));
        }
        None
    }

    fn scan_latex(&self) -> Option<usize> {
        let c = self.peek(0)?;

        if c == '%' {
            return self.input[self.cursor..]
                .find('\n')
                .map(|idx| self.cursor + idx)
                .or(Some(self.input.len()));
        }

        if c == '$' {
            let is_double = self.peek(1) == Some('$');
            let offset = if is_double { 2 } else { 1 };
            let pattern = if is_double { "$$" } else { "$" };
            return self.input[self.cursor + offset..]
                .find(pattern)
                .map(|idx| self.cursor + offset + idx + pattern.len());
        }

        if c == '\\' {
            let bytes = self.input.as_bytes();
            let mut i = 1;
            while self.cursor + i < bytes.len() && (bytes[self.cursor + i] as char).is_alphabetic()
            {
                i += 1;
            }
            let cmd_name = &self.input[self.cursor + 1..self.cursor + i];

            if cmd_name == "begin" {
                if let Some(env_name) = self.extract_braced_content(self.cursor + i) {
                    if LATEX_VERBATIM_ENVIRONMENTS.contains(&env_name) {
                        let closer = format!("\\end{{{}}}", env_name);
                        if let Some(close_pos) = self.input[self.cursor..].find(&closer) {
                            return Some(self.cursor + close_pos + closer.len());
                        }
                    }
                }
            }

            if LATEX_STRUCTURAL_COMMANDS.contains(&cmd_name) {
                return self.scan_balanced_braces(self.cursor + i);
            }

            return Some(self.cursor + i);
        }
        None
    }

    fn extract_braced_content(&self, start_search: usize) -> Option<&'a str> {
        let bytes = self.input.as_bytes();
        let mut curr = start_search;

        while curr < bytes.len() {
            let c = bytes[curr] as char;
            if c == '{' {
                break;
            }
            if c.is_whitespace() || c == '[' || c == ']' {
                curr += 1;
                continue;
            }
            return None;
        }

        if curr >= bytes.len() {
            return None;
        }

        let start_content = curr + 1;
        if let Some(end_offset) = self.input[start_content..].find('}') {
            return Some(&self.input[start_content..start_content + end_offset]);
        }
        None
    }

    fn scan_balanced_braces(&self, start: usize) -> Option<usize> {
        let bytes = self.input.as_bytes();
        let mut curr = start;

        loop {
            while curr < bytes.len() && (bytes[curr] as char).is_whitespace() {
                curr += 1;
            }
            if curr >= bytes.len() {
                break;
            }

            let next_char = bytes[curr] as char;
            if next_char == '{' || next_char == '[' {
                let closer = if next_char == '{' { '}' } else { ']' };
                let mut depth = 1;
                let mut j = 1;

                while curr + j < bytes.len() && depth > 0 {
                    let cc = bytes[curr + j] as char;
                    if cc == next_char {
                        depth += 1;
                    } else if cc == closer {
                        depth -= 1;
                    }
                    j += 1;
                }
                curr += j;
            } else {
                break;
            }
        }
        Some(curr)
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(chunk) = self.buffer.pop_front() {
            return Some(chunk);
        }
        self.scan_next()
    }
}

// --- UTILS ---

fn is_word_char(text: &str, idx: usize) -> bool {
    text.as_bytes().get(idx).map_or(false, |&b| {
        let c = b as char;
        c.is_alphanumeric() || c == '_'
    })
}

fn find_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    let needle_len = needle_bytes.len();

    if haystack_bytes.len() < needle_len {
        return None;
    }

    // Use char_indices ONLY to ensure we start matching at a valid character boundary.
    // However, we perform the match on BYTES to avoid panicking if the match
    // extends into the middle of a multibyte character.
    for (i, _) in haystack.char_indices() {
        if i + needle_len > haystack_bytes.len() {
            break;
        }

        // This slice is safe because we are slicing a &[u8], not a &str.
        // It doesn't care about UTF-8 boundaries.
        if haystack_bytes[i..i + needle_len].eq_ignore_ascii_case(needle_bytes) {
            return Some(i);
        }
    }
    None
}
