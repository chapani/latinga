use crate::Tartib;
use crate::trie::Trie;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashSet;

// Embedded Default Dictionaries
const DEFAULT_SUFFIXES: &str = include_str!("../dict/qoshimchalar.txt");
const DEFAULT_PROPER_NOUNS: &str = include_str!("../dict/atoqlilar.txt");
const DEFAULT_SUBSTITUTIONS: &str = include_str!("../dict/almashuvchilar.txt");
const DEFAULT_HEALS: &str = include_str!("../dict/tuzatishlar.txt");
const DEFAULT_NIQOBS: &str = include_str!("../dict/qalqonlar.txt");

pub struct Dictionary {
    // Group A: High-performance fixed string matcher
    pub qalqons_ac: Option<AhoCorasick>,

    // Group B: Complex patterns that require full regex engine
    pub qalqons_re: Vec<Regex>,

    // Group C: Persistent storage for literals (needed to rebuild AC)
    pub qalqons_literals: Vec<String>,

    pub substitutions_trie: Trie,
    pub healing_trie: Trie,
    pub proper_nouns_trie: Trie,
    pub suffixes: HashSet<String>,

    // Fast boolean filters (Bloom-filter style optimization for first char)
    pub healing_first_chars: [bool; 256],
    pub proper_noun_first_chars: [bool; 256],
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl Dictionary {
    #[must_use]
    pub fn new() -> Self {
        Self {
            qalqons_ac: None,
            qalqons_re: Vec::new(),
            qalqons_literals: Vec::new(),
            substitutions_trie: Trie::new(),
            healing_trie: Trie::new(),
            proper_nouns_trie: Trie::new(),
            suffixes: HashSet::new(),
            healing_first_chars: [false; 256],
            proper_noun_first_chars: [false; 256],
        }
    }

    pub fn load_defaults(&mut self, mode: Tartib) {
        self.load_suffixes(DEFAULT_SUFFIXES);
        if mode == Tartib::Kelgusi {
            self.load_proper_nouns(DEFAULT_PROPER_NOUNS);
        }
        self.load_substitutions(DEFAULT_SUBSTITUTIONS);
        if mode == Tartib::Joriy {
            self.load_healing(DEFAULT_HEALS);
        }
        // We ignore errors on default qalqons because we know the file is valid at compile time
        let _ = self.load_qalqons(DEFAULT_NIQOBS);
    }

    // --- LOADING LOGIC ---

    pub fn load_proper_nouns(&mut self, content: &str) {
        for name in Self::parse_lines(content) {
            if let Some(first_char) = name.chars().next() {
                let idx = first_char.to_ascii_lowercase() as usize;
                if idx < 256 {
                    self.proper_noun_first_chars[idx] = true;
                }
            }
            // Store lowercase key for case-insensitive lookup, preserve original value
            self.proper_nouns_trie.insert(&name.to_lowercase(), name);
        }
    }

    pub fn load_substitutions(&mut self, content: &str) {
        for line in Self::parse_lines(content) {
            if let Some((cyr, lat)) = line.split_once(':') {
                let cyr_lower = cyr.trim().to_lowercase();
                let lat_clean = lat
                    .trim()
                    .replace(
                        &crate::symbols::BARCHA_TUTUQ_TURLARI[..],
                        &crate::symbols::TUTUQ.to_string(),
                    )
                    .to_lowercase();

                self.substitutions_trie.insert(&cyr_lower, &lat_clean);
            }
        }
    }

    pub fn load_healing(&mut self, content: &str) {
        for line in Self::parse_lines(content) {
            // "broken" is the key (input text), "line" is the value (correct replacement)
            // Example: "o'zbek" -> "o‘zbek"
            let broken = line
                .replace(&crate::symbols::BARCHA_TUTUQ_TURLARI[..], "")
                .to_lowercase();

            // Populate the Fast Filter
            if let Some(first_char) = broken.chars().next() {
                if (first_char as usize) < 256 {
                    self.healing_first_chars[first_char as usize] = true;
                }
            }
            self.healing_trie.insert(&broken, line);
        }
    }

    pub fn load_suffixes(&mut self, content: &str) {
        for line in Self::parse_lines(content) {
            self.suffixes.insert(line.to_lowercase());
        }
    }

    pub fn load_qalqons(&mut self, content: &str) -> Result<(), regex::Error> {
        let regex_chars = ['\\', '[', ']', '(', ')', '*', '?', '+', '^', '$', '{', '}'];

        // 1. Parse and separate
        for line in Self::parse_lines(content) {
            let is_regex = line.chars().any(|c| regex_chars.contains(&c));

            if is_regex {
                self.qalqons_re.push(Regex::new(line)?);
            } else {
                self.qalqons_literals.push(line.to_string());
            }
        }

        // 2. Rebuild the Automaton with ALL literals
        if !self.qalqons_literals.is_empty() {
            let ac = AhoCorasickBuilder::new()
                .ascii_case_insensitive(true)
                .match_kind(MatchKind::LeftmostLongest)
                .build(&self.qalqons_literals)
                .map_err(|e| regex::Error::Syntax(e.to_string()))?;

            self.qalqons_ac = Some(ac);
        }

        Ok(())
    }

    // --- LOOKUP LOGIC ---

    #[must_use]
    pub fn find_stem_match(&self, word: &str) -> Option<usize> {
        self.proper_nouns_trie
            .find_longest_prefix(word)
            .map(|(len, _)| len)
    }

    fn parse_lines(content: &str) -> impl Iterator<Item = &str> {
        // Clean BOM *before* splitting into lines to handle Windows files
        Self::clean_bom(content)
            .lines()
            .map(|l| l.split('#').next().unwrap_or("").trim())
            .filter(|l| !l.is_empty())
    }

    fn clean_bom(input: &str) -> &str {
        input.strip_prefix('\u{FEFF}').unwrap_or(input)
    }

    /// "Heals" text by correcting common mistakes (e.g., missing apostrophes).
    /// RETURNS COW: Zero allocations if no healing is performed.
    #[must_use]
    pub fn heal<'a>(&self, text: &'a str) -> Cow<'a, str> {
        // 1. OPTIMIZATION: Zero-Copy Check for Ghost Marks
        // Only allocate a new string if the ghost mark (combining comma) is actually present.
        let cleaned_cow = if text.contains(crate::symbols::GHOST_MARK) {
            Cow::Owned(text.replace(
                crate::symbols::GHOST_MARK,
                &crate::symbols::OKINA.to_string(),
            ))
        } else {
            Cow::Borrowed(text)
        };

        // If the dictionary is empty, we are done.
        if self.healing_trie.is_empty() {
            return cleaned_cow;
        }

        // 2. OPTIMIZATION: Check if we even need to scan
        // If the string doesn't contain any characters that start our healing words, return early.
        // This requires scanning the text, which might be expensive for long texts.
        // Assuming text is short-ish (tokens) or we rely on the Trie to be fast.

        let cleaned_text = cleaned_cow.as_ref();

        // We will build `result` only upon the FIRST match.
        let mut result: Option<String> = None;
        let mut last_processed_idx = 0;
        let mut i = 0;
        let mut last_char: Option<char> = None;

        while i < cleaned_text.len() {
            let rest = &cleaned_text[i..];
            let c = rest.chars().next().unwrap();

            // Fast O(1) Boundary Check (Start of word)
            let is_boundary = last_char.map_or(true, |lc| !lc.is_alphabetic());

            if is_boundary && c.is_alphabetic() {
                // Try to find a healing match
                if let Some((match_len, matched_fragment)) = self.try_heal_token(rest, c) {
                    // MATCH FOUND!
                    // Initialize result buffer if this is the first match
                    if result.is_none() {
                        let mut s = String::with_capacity(cleaned_text.len() + 8);
                        s.push_str(&cleaned_text[..i]); // Push everything up to here
                        result = Some(s);
                    }

                    if let Some(ref mut res) = result {
                        res.push_str(&matched_fragment);
                    }

                    // Advance cursor
                    last_processed_idx = i + match_len;
                    let original_fragment = &cleaned_text[i..i + match_len];
                    last_char = original_fragment.chars().last();
                    i += match_len;
                    continue;
                }
            }

            // No match found at this position.
            // If we have a result buffer, we need to push the current char manually
            // to keep it in sync. If we don't, we just skip over (Zero Copy).
            if let Some(ref mut res) = result {
                res.push(c);
            }

            last_char = Some(c);
            i += c.len_utf8();
        }

        // Return logic
        match result {
            Some(res) => {
                // Push any remaining tail
                if last_processed_idx < cleaned_text.len() && last_processed_idx > i {
                    // This case implies i went backwards? No.
                    // i is at end. last_processed is where we last wrote.
                    // But we wrote char-by-char in the loop if result existed.
                    // Wait, in the loop: "If we have a result buffer... res.push(c)".
                    // So we are sync.
                }
                Cow::Owned(res)
            }
            None => cleaned_cow, // Return the original Cow (Borrowed or Owned from Ghost mark)
        }
    }

    #[inline]
    fn try_heal_token(&self, text_slice: &str, first_char: char) -> Option<(usize, String)> {
        // 3. OPTIMIZATION: Fast boolean array filter
        let c_idx = first_char.to_ascii_lowercase() as usize;
        let might_heal = if c_idx < 256 {
            self.healing_first_chars[c_idx]
        } else {
            true // Always check non-ASCII chars
        };

        if might_heal {
            if let Some((match_bytes, replacement)) =
                self.healing_trie.find_longest_prefix(text_slice)
            {
                let original_fragment = &text_slice[..match_bytes];
                return Some((
                    match_bytes,
                    Self::match_case(original_fragment, replacement),
                ));
            }
        }
        None
    }

    pub fn match_case(original: &str, replacement: &str) -> String {
        let mut chars = original.chars();
        let first = chars.next();

        let starts_upper = first.is_some_and(char::is_uppercase);

        if !starts_upper {
            return replacement.to_lowercase();
        }

        let is_all_upper = chars.all(char::is_uppercase);

        if is_all_upper {
            return replacement.to_uppercase();
        }

        // Title Case
        let mut r = String::with_capacity(replacement.len());
        let mut rep_chars = replacement.chars();

        if let Some(f) = rep_chars.next() {
            for c in f.to_uppercase() {
                r.push(c);
            }
            r.push_str(rep_chars.as_str());
        }
        r
    }
}
