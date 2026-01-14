#![cfg(not(target_arch = "wasm32"))]

use latinga::{ODATIY_TIRNOQ, OKINA, Oegirgich, TESKARI_TIRNOQ, TUTUQ, Tartib};

mod common;

// --- Group 1: Core Linguistic & Phonetic Logic (Joriy Mode) ---

#[test]
fn test_joriy_alphabet_mappings_and_casing() {
    let tr = common::setup_translator(Tartib::Joriy, None);

    // Standard Alphabet Mappings
    assert_eq!(tr.oegir("ЯХШИ"), "YAXSHI");
    assert_eq!(tr.oegir("Ёш"), "Yosh");
    assert_eq!(tr.oegir("АҚШ"), "AQSH");
    assert_eq!(tr.oegir("ЧАТ"), "CHAT");
    assert_eq!(tr.oegir("Шарқ"), "Sharq");

    // Standardizing input apostrophes for O' and G'
    let input = format!("o{ODATIY_TIRNOQ}rdak g{TESKARI_TIRNOQ}oz");
    let expected = format!("o{OKINA}rdak g{OKINA}oz");
    assert_eq!(tr.oegir(&input), expected);
}

#[test]
fn test_joriy_phonetic_rules_for_vowels_and_soft_signs() {
    let tr = common::setup_translator(Tartib::Joriy, None);

    // 'E' Rules & Boundaries (Start of word vs internal)
    assert_eq!(tr.oegir("Ер"), "Yer");
    assert_eq!(tr.oegir("Океан"), "Okean");
    assert_eq!(tr.oegir("Мен"), "Men");
    assert_eq!(tr.oegir("Бу ер"), "Bu yer");
    assert_eq!(tr.oegir("«Ер»"), "«Yer»");
    assert_eq!(tr.oegir("—Ер"), "—Yer");
    assert_eq!(tr.oegir("(\"Ер\")"), "(\"Yer\")");

    // 'TS' (Ц) and Soft Sign (Ь)
    assert_eq!(tr.oegir("Цирк"), "Sirk");
    assert_eq!(tr.oegir("Рация"), "Ratsiya");
    assert_eq!(tr.oegir("Станция"), "Stansiya");
    assert_eq!(tr.oegir("Кварц"), "Kvars");
    assert_eq!(tr.oegir("лагерь"), "lager");
    assert_eq!(tr.oegir("фильм"), "film");
}

#[test]
fn test_joriy_glottal_stops_and_phonetic_syllable_separation() {
    let tr = common::setup_translator(Tartib::Joriy, None);

    // Ayirish Belgisi (ъ) -> TUTUQ/O_OKINA
    assert_eq!(tr.oegir("Маъно"), format!("Ma{TUTUQ}no"));
    assert_eq!(tr.oegir("Раъно"), format!("Ra{TUTUQ}no"));
    assert_eq!(tr.oegir("Шеър"), format!("She{TUTUQ}r"));
    assert_eq!(tr.oegir("Мўъжиза"), format!("Mo{OKINA}jiza"));
    assert_eq!(tr.oegir("қитъа"), format!("qit{TUTUQ}a"));
    assert_eq!(tr.oegir("съезд"), "syezd");
    assert_eq!(tr.oegir("Объект"), "Obyekt");
    assert_eq!(tr.oegir("манъг"), format!("man{TUTUQ}g"));

    // Syllable Separation for S+H
    assert_eq!(tr.oegir("Ishoq"), format!("Is{TUTUQ}hoq"));
    assert_eq!(tr.oegir("ishoq"), format!("is{TUTUQ}hoq"));
    assert_eq!(tr.oegir("ISHOQ"), format!("IS{TUTUQ}HOQ"));
    assert_eq!(tr.oegir("Mushaf"), format!("Mus{TUTUQ}haf"));
    assert_eq!(tr.oegir("mushaf"), format!("mus{TUTUQ}haf"));
    assert_eq!(tr.oegir("MUSHAF"), format!("MUS{TUTUQ}HAF"));

    // Standardizing existing separators
    let input_sh = format!("is{ODATIY_TIRNOQ}hoq");
    assert_eq!(tr.oegir(&input_sh), format!("is{TUTUQ}hoq"));

    assert_eq!(tr.oegir("исҳоқ"), format!("is{TUTUQ}hoq"));
    assert_eq!(tr.oegir("қашшоқ"), "qashshoq"); // Negative check (digraph)
}

// --- Group 2: Dictionary Healing & Normalization ---

#[test]
fn test_joriy_dictionary_healing_for_stems() {
    let tr = Oegirgich::fitrat_ila_yangi(Tartib::Joriy);

    // Contextual Glottal Healing
    assert_eq!(tr.oegir("manosini"), format!("ma{TUTUQ}nosini"));
    assert_eq!(tr.oegir("Manosini"), format!("Ma{TUTUQ}nosini"));
    assert_eq!(tr.oegir("MANOSINI"), format!("MA{TUTUQ}NOSINI"));
    assert_eq!(tr.oegir("juratini"), format!("jur{TUTUQ}atini"));
    assert_eq!(tr.oegir("Juratini"), format!("Jur{TUTUQ}atini"));
    assert_eq!(tr.oegir("JURATINI"), format!("JUR{TUTUQ}ATINI"));
}

#[test]
fn test_joriy_apostrophe_and_quote_standardization() {
    let tr = common::setup_translator(Tartib::Joriy, None);

    // O' Normalization
    let input_o = format!("o{ODATIY_TIRNOQ}zbek");
    assert_eq!(tr.oegir(&input_o), format!("o{OKINA}zbek"));

    // G' Normalization
    let input_g = format!("g{TESKARI_TIRNOQ}alati");
    assert_eq!(tr.oegir(&input_g), format!("g{OKINA}alati"));

    // General Glottal Stop Normalization
    let input_ma = format!("ma{ODATIY_TIRNOQ}rifat");
    assert_eq!(tr.oegir(&input_ma), format!("ma{TUTUQ}rifat"));

    // S'H Normalization
    let input_mus = format!("Mus{ODATIY_TIRNOQ}haf");
    assert_eq!(tr.oegir(&input_mus), format!("Mus{TUTUQ}haf"));

    // Quote preservation vs digraph standardization
    let input_q = format!(
        "Rus tilida 'shahar' so{}zi juda muhim.",
        crate::ODATIY_TIRNOQ
    );
    let expected_q = format!("Rus tilida 'shahar' so{}zi juda muhim.", crate::OKINA);
    assert_eq!(tr.oegir(&input_q), expected_q);
}

// --- Group 3: Technical Shielding (LaTeX & Web) ---

#[test]
fn test_latex_protection_and_math_integrity_joriy() {
    let tr = common::setup_translator(Tartib::Joriy, None);

    // 1. Math block preservation
    let input_math = r"\section{Кирилл} ва $x^2 + y = 1$";
    let expected_math = r"\section{Kirill} va $x^2 + y = 1$";
    assert_eq!(tr.oegir(input_math), expected_math);

    // 2. Syntax and Command preservation
    let input_cmd = r"\textbf{Исҳоқ} ва $y^s$";
    let expected_cmd = format!(r"\textbf{{Is{TUTUQ}hoq}} va $y^s$");
    assert_eq!(tr.oegir(input_cmd), expected_cmd);

    // 3. Math Derivatives (Prime symbol) vs Tutuq
    let input_prime = "$f'(x) = y$ ва маъно";
    let expected_prime = format!("$f'(x) = y$ va ma{TUTUQ}no");
    assert_eq!(tr.oegir(input_prime), expected_prime);

    // 4. Escaped Symbols
    // FIXED: The surrounding text "ва" MUST be translated to "va"
    let input_sym = r"\& ва \'";
    let expected_sym = r"\& va \'";
    assert_eq!(tr.oegir(input_sym), expected_sym);
}

#[test]
fn test_latex_package_and_option_shielding_joriy() {
    let tr = common::setup_translator(Tartib::Joriy, None);
    // Verifies technical Latin options are skipped
    let input = r#"\usepackage[english]{babel}"#;
    assert_eq!(tr.oegir(input), input);
}

#[test]
fn test_universal_shield_tag_interaction_joriy() {
    let tr = common::setup_translator(Tartib::Joriy, None);

    // Inline Shielding {] [}
    let input = "Ер ва {]Ер[}.";
    let expected = "Yer va Ер.";
    assert_eq!(tr.oegir(input), expected);

    // Multiline Shielding
    let input_multi = "{]\nЮлдуз\nЯнвар\n[}";
    let expected_multi = "\nЮлдуз\nЯнвар\n";
    assert_eq!(tr.oegir(input_multi), expected_multi);
}

#[test]
fn test_web_surgical_attribute_unmasking_joriy() {
    let tr = common::setup_translator(Tartib::Joriy, None);

    // Meta content transliteration
    let input_desc = r#"<meta name="description" content="Ғолиблар президент совғаси...">"#;
    let expected_desc = r#"<meta name="description" content="Gʻoliblar prezident sovgʻasi...">"#;
    assert_eq!(tr.oegir(input_desc), expected_desc);

    // Tag integrity preservation
    let input_title = r#"<meta name="title" content="Ғолиблар">"#;
    let expected_title = r#"<meta name="title" content="Gʻoliblar">"#;
    assert_eq!(tr.oegir(input_title), expected_title);
}

// --- Group 4: Validation & Diagnostics ---

#[test]
fn test_joriy_validation_apostrophe_and_ambiguity_reporting() {
    let tr = common::setup_translator(Tartib::Joriy, None);

    // 1. Invalid Apostrophe
    let invalid_input = format!("o{ODATIY_TIRNOQ}rdak");
    let result_o = tr.tekshir(&invalid_input, 3);
    assert_eq!(result_o.hatolar.len(), 1);
    assert_eq!(result_o.hatolar[0].soez, "o'rdak");

    // 2. Syllabic Ambiguity
    let result_sh = tr.tekshir("Ishoq", 3);
    assert_eq!(result_sh.hatolar.len(), 1);
    assert!(result_sh.hatolar[0].habar.to_lowercase().contains("tutuq"));
}

#[test]
fn test_joriy_validation_multiline_coordinates() {
    let tr = common::setup_translator(Tartib::Joriy, None);
    let input = format!("Yaxshi\nma{ODATIY_TIRNOQ}no\nan{TESKARI_TIRNOQ}ana");
    let result = tr.tekshir(&input, 1);

    assert_eq!(result.jami, 2);
    assert_eq!(result.hatolar.len(), 1);
    assert_eq!(result.hatolar[0].qator, 2);
    assert_eq!(result.hatolar[0].ustun, 3);
}

#[test]
fn test_joriy_validation_boundary_regression_cases() {
    let tr = common::setup_translator(Tartib::Joriy, None);

    // Regression: Infinite loop on certain word patterns
    let input_loop = "ma'nosi nima o'zi?";
    assert_eq!(tr.tekshir(input_loop, 3).hatolar.len(), 2);

    // Regression: Newline at end of text
    let input_nl = "o'rdak\n";
    assert_eq!(tr.tekshir(input_nl, 3).hatolar.len(), 1);
}
