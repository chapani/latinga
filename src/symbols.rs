pub const OKINA: char = '\u{02BB}'; // ʻ
pub const OKINA_STR: &str = "\u{02BB}";

pub const TUTUQ: char = '\u{02BC}'; // ʼ
pub const TUTUQ_STR: &str = "\u{02BC}";

// Standard Punctuation
pub const YOPUVCHI_TIRNOQ: char = '\u{2019}'; // ’
pub const OCHUVCHI_TIRNOQ: char = '\u{2018}'; // ‘
pub const ODATIY_TIRNOQ: char = '\u{0027}'; // '
pub const TESKARI_TIRNOQ: char = '\u{0060}'; // `
pub const URGU: char = '\u{00B4}'; // ´
pub const GHOST_MARK: char = '\u{0312}'; // Combining comma above

pub const BARCHA_TUTUQ_TURLARI: [char; 7] = [
    ODATIY_TIRNOQ,
    TESKARI_TIRNOQ,
    YOPUVCHI_TIRNOQ,
    OCHUVCHI_TIRNOQ,
    URGU,
    OKINA,
    TUTUQ,
];

pub const CYR_VOWELS: &str = "аеёиоуэюяўыАЕЁИОУЭЮЯЎЫ";

/// Mappings for the Kelgusi (Future) Latin alphabet reform.
pub static KELGUSI_MAP: &[(&str, &str)] = &[
    ("g'", "ğ"),
    ("gʻ", "ğ"),
    ("g`", "ğ"),
    ("g‘", "ğ"),
    ("g’", "ğ"),
    ("o'", "ö"),
    ("oʻ", "ö"),
    ("o`", "ö"),
    ("o‘", "ö"),
    ("o’", "ö"),
    ("sh", "ş"),
    ("ch", "ç"),
];

// Optimized lookup table for 1-to-1 Cyrillic -> Latin mapping
pub const MAP_1_TO_1: &[(char, &str)] = &[
    ('А', "A"),
    ('а', "a"),
    ('Б', "B"),
    ('б', "b"),
    ('В', "V"),
    ('в', "v"),
    ('Г', "G"),
    ('г', "g"),
    ('Д', "D"),
    ('д', "d"),
    ('Ж', "J"),
    ('ж', "j"),
    ('З', "Z"),
    ('з', "z"),
    ('И', "I"),
    ('и', "i"),
    ('Й', "Y"),
    ('й', "y"),
    ('К', "K"),
    ('к', "k"),
    ('Л', "L"),
    ('л', "l"),
    ('М', "M"),
    ('м', "m"),
    ('Н', "N"),
    ('н', "n"),
    ('О', "O"),
    ('о', "o"),
    ('П', "P"),
    ('п', "p"),
    ('Р', "R"),
    ('р', "r"),
    ('С', "S"),
    ('с', "s"),
    ('Т', "T"),
    ('т', "t"),
    ('У', "U"),
    ('у', "u"),
    ('Ф', "F"),
    ('ф', "f"),
    ('Қ', "Q"),
    ('қ', "q"),
    ('Ҳ', "H"),
    ('ҳ', "h"),
    ('Ы', "I"),
    ('ы', "i"),
    ('Э', "E"),
    ('э', "e"),
];

pub const TRANSLITERABLE_ATTRIBUTES: &[&str] = &["content", "title", "alt", "placeholder", "label"];

pub const FULLY_PROTECTED_TAGS: &[&str] = &["script", "style", "code", "pre"];
