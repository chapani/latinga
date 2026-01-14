#![cfg(not(target_arch = "wasm32"))]

use latinga::{ODATIY_TIRNOQ, OKINA, Oegirgich, Sozlama, TESKARI_TIRNOQ, TUTUQ, Tartib};
use std::fs;
use tempfile::tempdir;

// Links to tests/common/mod.rs
mod common;

// --- Group 1: Shielding & Content Protection ---

#[test]
fn test_shield_protects_markdown_and_code_blocks() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    // Code and Format Protection
    assert_eq!(tr.oegir("Mana `code_sh` bloki"), "Mana `code_sh` bloki");
    assert_eq!(tr.oegir("```\nshahar_nomi\n```"), "```\nshahar_nomi\n```");

    // Mixed context: code block vs normal text
    let input_code = format!("Asosiy `shahar_nomi` o{ODATIY_TIRNOQ}zgarmasin.");
    assert_eq!(tr.oegir(&input_code), "Asosiy `shahar_nomi` özgarmasin.");

    // Comments
    assert_eq!(tr.oegir("/* shahar */"), "/* şahar */");
}

#[test]
fn test_shield_protects_web_structures_and_emails() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    // Markdown Links
    let link = "[shahar](https://uz.wikipedia.org/wiki/shahar)";
    assert_eq!(
        tr.oegir(link),
        "[şahar](https://uz.wikipedia.org/wiki/shahar)"
    );

    // HTML Structure
    assert_eq!(
        tr.oegir("<div class=\"shahar\">choy</div>"),
        "<div class=\"shahar\">çoy</div>"
    );

    // HTML Entities and special characters
    let input_html = format!("Bo{TESKARI_TIRNOQ}sh&shy;liq");
    assert_eq!(tr.oegir(&input_html), "Böş&shy;liq");

    // Contact Info (Emails)
    assert_eq!(tr.oegir("Email: info@latinga.uz"), "Email: info@latinga.uz");
}

// --- Group 2: Dictionary & Qalqon (Masking) ---

#[test]
fn test_qalqon_masking_overrides_translation_logic() {
    let mut config = Sozlama::yangi(Tartib::Kelgusi);
    let qalqon_content = "# Comment\nXerox # Brand\nLinux";
    config.qalqonlarni_yukla(qalqon_content).unwrap();

    let tr = Oegirgich::yangi(config);
    let input = "Xerox компанияси Linux тизимидан фойдаланади.";
    let result = tr.oegir(input);

    assert!(result.contains("Xerox"));
    assert!(result.contains("Linux"));
    assert!(result.contains("kompaniyasi"));
}

#[test]
fn test_validator_respects_custom_shields_both_modes() {
    let qalqons = r"\*\*Qalqon:\*\* ([^\n]+)";

    let joriy = common::setup_translator(Tartib::Joriy, Some(qalqons));

    assert_eq!(
        joriy.tekshir("O'rdak", 10).jami,
        1,
        "Joriy: Unshielded straight quote should be an error"
    );

    assert_eq!(
        joriy.tekshir("**Qalqon:** O'rdak", 10).jami,
        0,
        "Joriy: Shielded straight quote should be ignored"
    );

    let kelgusi = common::setup_translator(Tartib::Kelgusi, Some(qalqons));

    assert_eq!(
        kelgusi.tekshir("shahar", 10).jami,
        1,
        "Kelgusi: Unshielded 'sh' should be an error"
    );

    assert_eq!(
        kelgusi.tekshir("**Qalqon:** shahar", 10).jami,
        0,
        "Kelgusi: Shielded 'sh' should be ignored"
    );
}

#[test]
fn test_dictionary_loading_from_csv_conversion() {
    let processed_input = "Rust, NATO, al-Horazmiy".replace(',', "\n");

    let mut config = Sozlama::yangi(Tartib::Kelgusi);
    config.atoqlilarni_yukla(&processed_input);

    let t = Oegirgich::yangi(config);

    assert_eq!(t.oegir("Rustda"), "Rust'da");
    assert_eq!(t.oegir("NATOda"), "NATO'da");
    assert_eq!(t.oegir("al-Horazmiyni"), "al-Horazmiy'ni");
}

// --- Group 3: File System & Workflow Integration ---

#[test]
fn test_file_system_read_write_consistency() {
    let dir = tempdir().unwrap();
    let tr = common::setup_translator(Tartib::Joriy, None);

    let file_path = dir.path().join("test_overwrite.txt");
    fs::write(&file_path, "Маъно ва эътиқод").unwrap();

    let content = fs::read_to_string(&file_path).unwrap();
    fs::write(&file_path, tr.oegir(&content)).unwrap();

    // EXPECTED: 'ъ' after a vowel in Joriy must become the TUTUQ constant
    let expected_joriy = format!("Ma{TUTUQ}no va e{TUTUQ}tiqod");
    assert_eq!(fs::read_to_string(&file_path).unwrap(), expected_joriy);
}

#[test]
fn test_glob_pattern_matching_in_virtual_directory() {
    let dir = tempdir().unwrap();
    let docs_path = dir.path().join("docs");
    fs::create_dir(&docs_path).unwrap();
    fs::write(docs_path.join("1.txt"), "Кирилл").unwrap();

    let pattern = format!("{}/docs/*.txt", dir.path().to_str().unwrap());
    let entries: Vec<_> = glob::glob(&pattern)
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    assert_eq!(entries.len(), 1);
}

// --- Group 4: Core Engine Integrity ---

#[test]
fn test_engine_transliteration_accuracy() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    assert_eq!(tr.oegir("Вашингтон va choy"), "Vaşington va çoy");
    assert_eq!(tr.oegir("X asr"), "X asr");
    assert_eq!(tr.oegir("Хизмат"), "Hizmat");

    let def_tr = Oegirgich::fitrat_ila_yangi(Tartib::Kelgusi);
    assert_eq!(def_tr.oegir("Linux"), "Linux");
}

#[test]
fn test_engine_unicode_normalization_healing() {
    // --- Unicode Normalization & Healing Test ---
    // \u{0312} is the 'Combining Turned Comma Above'.
    const COMBINING_TURNED_COMMA: char = '\u{0312}';
    let joriy_tr = common::setup_translator(Tartib::Joriy, None);

    let input = format!("O{COMBINING_TURNED_COMMA}");
    let expected = format!("O{OKINA}");

    assert_eq!(joriy_tr.oegir(&input), expected);
}
