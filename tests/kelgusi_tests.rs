#![cfg(not(target_arch = "wasm32"))]

use latinga::{ODATIY_TIRNOQ, OKINA, Oegirgich, Sozlama, TUTUQ, Tartib};
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

mod common;

// --- Group 1: Core Linguistic Logic (Kelgusi/Proposed Alphabet) ---

#[test]
fn test_kelgusi_alphabet_mappings_and_glottal_stops() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    // Basic Alphabet Mappings (Digraphs to single letters)
    let input_basic = format!("shahar choy o{OKINA}rdak g{OKINA}ildirak");
    assert_eq!(tr.oegir(&input_basic), "şahar çoy ördak ğildirak");

    // Glottal Stop Removal (Proposed rule: removal of apostrophe in specific contexts)
    let input_ma = format!("Prezidentning ma{TUTUQ}ruzasi");
    assert_eq!(tr.oegir(&input_ma), "Prezidentning maruzasi");

    // Ayirish Belgisi (ъ) Handling
    assert_eq!(tr.oegir("Маъно"), "Mano");
    assert_eq!(tr.oegir("Шеър"), "Şer");
    assert_eq!(tr.oegir("Мўъжиза"), "Möjiza");
    assert_eq!(tr.oegir("қитъа"), "qita");
    assert_eq!(tr.oegir("съезд"), "syezd");
    assert_eq!(tr.oegir("Объект"), "Obyekt");
    assert_eq!(tr.oegir("манъг"), "mang");
    assert_eq!(tr.oegir("Ғани"), "Ğani");
}

#[test]
fn test_kelgusi_suffix_separation_for_proper_nouns() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    // Kelgusi mode uses ASCII_APOSTROPHE (') for suffix separation
    assert_eq!(tr.oegir("Toshkentda"), format!("Toşkent{ODATIY_TIRNOQ}da"));
    assert_eq!(
        tr.oegir("Toshkentdan keldim"),
        format!("Toşkent{ODATIY_TIRNOQ}dan keldim")
    );
    assert_eq!(
        tr.oegir("Самарқандга"),
        format!("Samarqand{ODATIY_TIRNOQ}ga")
    );
}

#[test]
fn test_hyphenated_proper_noun_separation() {
    // 1. Dictionary has hyphenated name
    let custom_dict = "Al-Horazmiy";

    let mut config = Sozlama::yangi(Tartib::Kelgusi);
    config.atoqlilarni_yukla(custom_dict);
    let t = Oegirgich::yangi(config);

    // 2. Input uses proper case.
    // Suffix "ning" should be separated.
    // If logic is broken, output is "Al-Horazmiyning"
    // If logic works, output is "Al-Horazmiy'ning"
    let input = "Al-Horazmiyning";
    let expected = "Al-Horazmiy'ning";

    assert_eq!(t.oegir(input), expected);
}

#[test]
fn test_kelgusi_strict_proper_noun_matching() {
    // Dict contains:
    // 1. "Rust" (Standard Title Case)
    // 2. "NATO" (Abbreviation)
    // 3. "al-Horazmiy" (Mixed Case / Hyphenated)
    // 4. "abc" (Lowercase proper noun)
    let custom_dict = "Rust\nNATO\nal-Horazmiy\nabc";

    let mut config = Sozlama::yangi(Tartib::Kelgusi);
    config.atoqlilarni_yukla(custom_dict);
    let t = Oegirgich::yangi(config);

    // 1. Rust vs rust
    assert_eq!(t.oegir("Rustda"), "Rust'da", "Matches 'Rust' -> Separate");
    assert_eq!(
        t.oegir("rustda"),
        "rustda",
        "Mismatch 'rust' -> No separator"
    );

    // 2. NATO vs nato
    assert_eq!(t.oegir("NATOda"), "NATO'da", "Matches 'NATO' -> Separate");
    assert_eq!(
        t.oegir("natoda"),
        "natoda",
        "Mismatch 'nato' -> No separator"
    );

    // 3. al-Horazmiy (Mixed Case Hyphenated)
    assert_eq!(
        t.oegir("al-Horazmiyni"),
        "al-Horazmiy'ni",
        "Matches 'al-Horazmiy' -> Separate"
    );
    // Incorrect casing input
    assert_eq!(
        t.oegir("Al-horazmiyni"),
        "Al-horazmiyni",
        "Mismatch 'Al-horazmiy' -> No separator"
    );

    // 4. abc (Lowercase proper noun)
    assert_eq!(t.oegir("abcda"), "abc'da", "Matches 'abc' -> Separate");
}

#[test]
fn test_kelgusi_complex_proper_noun_rules() {
    // Dict:
    // 1. "Rust" (Starts Upper -> Strict rules apply)
    // 2. "al-Horazmiy" (Starts Lower -> Loose rules apply)
    // 3. "Samarqand Universiteti" (Multi-word)
    let custom_dict = "Rust\nal-Horazmiy\nSamarqand Universiteti";

    let mut config = Sozlama::yangi(Tartib::Kelgusi);
    config.atoqlilarni_yukla(custom_dict);
    let t = Oegirgich::yangi(config);

    // --- 1. Capitalized in Dict ("Rust") ---
    // Match Original
    assert_eq!(t.oegir("Rustda"), "Rust'da", "Match 'Rust' (Exact)");
    // Match All-Caps
    assert_eq!(t.oegir("RUSTDA"), "RUST'DA", "Match 'RUST' (All Caps)");
    // Reject Lowercase
    assert_eq!(
        t.oegir("rustda"),
        "rustda",
        "Reject 'rust' (Common noun usage)"
    );

    // --- 2. Lowercase/Mixed in Dict ("al-Horazmiy") ---
    // Match Original
    assert_eq!(
        t.oegir("al-Horazmiyni"),
        "al-Horazmiy'ni",
        "Match 'al-Horazmiy' (Exact)"
    );
    // Match Title Case (Start of sentence) -> Preserve Input Case!
    assert_eq!(
        t.oegir("Al-Horazmiyni"),
        "Al-Horazmiy'ni",
        "Match 'Al-Horazmiy' (Title Case)"
    );
    // Match All Caps
    assert_eq!(
        t.oegir("AL-HORAZMIYNI"),
        "AL-HORAZMIY'NI",
        "Match 'AL-HORAZMIY' (All Caps)"
    );

    // --- 3. Multi-word Proper Noun ---
    // "Samarqand Universiteti" + "ga"
    assert_eq!(
        t.oegir("Samarqand Universitetiga"),
        "Samarqand Universiteti'ga",
        "Multi-word proper noun separation"
    );
}

#[test]
fn test_kelgusi_no_healing_glottal_stops() {
    // Kelgusi mode should NOT auto-heal "manosi" -> "ma'nosi" or "maʼnosi"
    // because the glottal stop character is dropped in the new alphabet.
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    let input = "manosi";
    let expected = "manosi"; // In Joriy, this would heal to "maʼnosi"

    let result = tr.oegir(input);

    assert_eq!(
        result, expected,
        "Kelgusi mode incorrectly healed a word! Expected '{}', got '{}'",
        expected, result
    );
}

// --- Group 2: Configuration & Dictionary Overrides ---

#[test]
fn test_kelgusi_custom_dictionary_and_suffix_loading() {
    let mut config = Sozlama::yangi(Tartib::Kelgusi);
    config.qoeshimchalarni_yukla("ga\ndan\nda");
    config.atoqlilarni_yukla("Elon\nMars");
    let tr = Oegirgich::yangi(config);

    assert_eq!(
        tr.oegir("Elonga ayting"),
        format!("Elon{ODATIY_TIRNOQ}ga ayting")
    );
}

#[test]
fn test_kelgusi_exception_file_overrides() {
    let mut user_file = NamedTempFile::new().unwrap();
    writeln!(user_file, "сентябрь:sentyabr").unwrap(); // Override default
    writeln!(user_file, "whatsapp:vatsap").unwrap(); // New entry

    let mut config = Sozlama::yangi(Tartib::Kelgusi);
    config.lughat.load_defaults(Tartib::Kelgusi);
    let extra_content = fs::read_to_string(user_file.path()).unwrap();
    config.almashuvchilarni_yukla(&extra_content);

    let tr = Oegirgich::yangi(config);

    assert_eq!(tr.oegir("Сентябрь"), "Sentyabr");
    assert_eq!(tr.oegir("WhatsApp"), "Vatsap");
    assert_eq!(tr.oegir("WHATSAPP"), "VATSAP");
    assert_eq!(tr.oegir("Январь"), "Yanvar"); // Verify chain still has defaults
}

// --- Group 3: LaTeX Shielding & Structure ---

#[test]
fn test_latex_prose_and_command_shielding() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    // Math commands (\cosh, \chi), environments (center), and prose
    let input = format!(r"\begin{{center}} \cosh(x) va \chi choy o{OKINA}rdak \end{{center}}");
    let expected = r"\begin{center} \cosh(x) va \chi çoy ördak \end{center}";
    assert_eq!(tr.oegir(&input), expected);

    // Special sequences (\#, \'o)
    let input_spec = r"Kelgusi \# va \'o \internal@command";
    assert_eq!(tr.oegir(input_spec), input_spec);
}

#[test]
fn test_latex_structural_references_and_labels() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    // Refs and Labels
    let input = r"Ushbu shahar \cite{shahar2024} juda go'zal. \label{shahar_ref} \includegraphics{shahar.png}";
    let expected =
        r"Uşbu şahar \cite{shahar2024} juda gözal. \label{shahar_ref} \includegraphics{shahar.png}";
    assert_eq!(tr.oegir(input), expected);

    assert_eq!(
        tr.oegir(r"Rasm: \ref{shahar_ref}"),
        r"Rasm: \ref{shahar_ref}"
    );
}

#[test]
fn test_latex_verbatim_and_environment_protection() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    // Verbatim block
    let input_verb = r#"Dastur kodi:
\begin{verbatim}
pub fn show_shahar() {
    my_func("shahar");
}
\end{verbatim}"#;
    assert_eq!(tr.oegir(input_verb), input_verb);

    // Package options and key-values
    assert_eq!(
        tr.oegir(r#"\usepackage[english]{babel}"#),
        r#"\usepackage[english]{babel}"#
    );
}

#[test]
fn test_latex_prose_conversion_in_sections() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    let mixed = r#"\section{Описание} % Description"#;
    let result = tr.oegir(mixed);
    assert!(result.starts_with(r#"\section{"#));
    assert!(result.contains("Opisaniye"));

    // Custom command collision
    let input = r#"\customcmd{shahar}"#;
    assert_eq!(tr.oegir(input), r#"\customcmd{şahar}"#);
}

// --- Group 4: Web & Shielding Integration ---

#[test]
fn test_universal_shield_tag_interaction() {
    let config = Sozlama::yangi(Tartib::Kelgusi);
    let translator = Oegirgich::yangi(config);

    // 1. Single Brace Shield {] [}
    let input_single = "Bu shahar va {]bu ham shahar[}.";
    assert_eq!(translator.oegir(input_single), "Bu şahar va bu ham shahar.");

    // 2. Double Brace Shield {{] [}} (System Context Standard)
    // Preserved from old test_textbook_scenario_kelgusi
    let input_double = format!("Rus tilida 'shahar' so{OKINA}zi {{]город[}} deb ataladi.");
    let expected_double = "Rus tilida 'şahar' sözi город deb ataladi.";
    assert_eq!(translator.oegir(&input_double), expected_double);

    // 3. Quotes and Mixed prose
    let input_q = format!("Rus tilida 'shahar' so{OKINA}zi juda muhim.");
    assert_eq!(
        translator.oegir(&input_q),
        "Rus tilida 'şahar' sözi juda muhim."
    );
}

#[test]
fn test_web_attribute_and_meta_tag_conversion() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    // Meta tags: content attribute should be converted
    let input_meta = r#"<meta name="description" content="Ғолиблар президент совғаси...">"#;
    let expected_meta = r#"<meta name="description" content="Ğoliblar prezident sovğasi...">"#;
    assert_eq!(tr.oegir(input_meta), expected_meta);

    // Surgical unmasking
    let input_surg = r#"<meta name="title" content="Ғолиблар">"#;
    let expected_surg = r#"<meta name="title" content="Ğoliblar">"#;
    assert_eq!(tr.oegir(input_surg), expected_surg);
}

// --- Group 5: Validation & Diagnostics ---

#[test]
fn test_validation_legacy_alphabet_detection() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    // Digraphs
    let res_sh = tr.tekshir("shahar", 3);
    assert!(res_sh.hatolar[0].habar.contains("'ş'"));

    let res_ch = tr.tekshir("olcha", 3);
    assert_eq!(res_ch.hatolar[0].ustun, 3);

    // Okinas
    let res_o = tr.tekshir("o'rdak", 3);
    assert!(res_o.hatolar[0].habar.contains("'ö'"));

    let res_g = tr.tekshir("g'isht", 3);
    assert!(res_g.hatolar[0].habar.contains("'ğ'"));
}

#[test]
fn test_validation_casing_and_multi_error_handling() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    // Full uppercase
    let res_upper = tr.tekshir("SHAXS", 3);
    assert!(res_upper.hatolar[0].habar.contains("'Ş'"));

    // Sentence case
    let res_sentence = tr.tekshir("Shaxs", 3);
    assert!(res_sentence.hatolar[0].habar.contains("'Ş'"));

    // Multiple errors
    let res_multi = tr.tekshir("balki u\nmashhur va olchoq", 1);
    assert_eq!(res_multi.jami, 2);
    assert_eq!(res_multi.hatolar.len(), 1);
    assert_eq!(res_multi.hatolar[0].soez, "mashhur");
    assert_eq!(res_multi.hatolar[0].qator, 2);
    assert_eq!(res_multi.hatolar[0].ustun, 3);
}

#[test]
fn test_validation_boundary_correctness() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);
    // 'ma'no' should NOT trigger legacy digraph error
    let res = tr.tekshir("ma'no", 3);
    assert_eq!(res.hatolar.len(), 0);
}
