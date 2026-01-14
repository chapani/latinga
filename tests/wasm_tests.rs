#![cfg(target_arch = "wasm32")]

use latinga::wasm::Latinga;
use serde::Deserialize;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// --- Helpers for Verification ---

// We define a mirror struct to deserialize the JS Object returned by tekshir()
// This ensures the data contract between Rust and JS is intact.
#[derive(Deserialize)]
struct TestSummary {
    jami: usize,
    hatolar: Vec<TestError>,
}

#[derive(Deserialize)]
struct TestError {
    qator: usize,
    ustun: usize,
    soez: String,
    habar: String,
}

// --- Group 1: Conversion Tests ---

#[wasm_bindgen_test]
fn test_joriy_conversion_cyrillic_to_latin() {
    // is_joriy = true
    let router = Latinga::yangi(true);

    // "School and Pupil" -> "Maktab va Oʻquvchi"
    let input = "Мактаб ва Ўқувчи";
    let output = router.oegir(input);

    assert_eq!(output, "Maktab va Oʻquvchi");
}

#[wasm_bindgen_test]
fn test_kelgusi_conversion_latin_digraphs() {
    // is_joriy = false
    let router = Latinga::yangi(false);

    // "shahar choy o'rdak" -> "şahar çoy ördak"
    let input = "shahar choy o'rdak";
    let output = router.oegir(input);

    assert_eq!(output, "şahar çoy ördak");
}

#[wasm_bindgen_test]
fn test_kelgusi_proper_noun_handling() {
    let mut router = Latinga::yangi(false);

    // "Google" is in default dict, but let's be explicit
    router.atoqlilarni_yukla("Google,Facebook");

    // "Googlega" -> "Google'ga" (Separation of suffix)
    let input = "Googlega";
    assert_eq!(router.oegir(input), "Google'ga");
}

#[wasm_bindgen_test]
fn test_router_persists_settings_across_calls() {
    let mut router = Latinga::yangi(false);

    // 1. Add Replacement
    router.almashuvchilarni_yukla("Foo:Bar");
    assert_eq!(router.oegir("Foo"), "Bar");

    // 2. Add Shield
    router.qalqonlarni_yukla(r"\[code\].*?\[/code\]");
    assert_eq!(
        router.oegir("Bu [code]shahar[/code]"),
        "Bu [code]shahar[/code]"
    );

    // 3. Verify Replacement still works
    assert_eq!(router.oegir("Foo"), "Bar");
}

// --- Group 2: Validation Tests ---

#[wasm_bindgen_test]
fn test_joriy_validation_structure() {
    let router = Latinga::yangi(true); // Joriy

    // "o'rdak" uses ASCII quote, which is strictly an error in Joriy (should be Oʻ)
    let input = "o'rdak";

    let js_val = router.tekshir(input, 10);
    let summary: TestSummary = serde_wasm_bindgen::from_value(js_val).unwrap();

    assert_eq!(summary.jami, 1);
    assert_eq!(summary.hatolar.len(), 1);

    let err = &summary.hatolar[0];
    assert_eq!(err.soez, "o'rdak");
    assert_eq!(err.qator, 1);
    assert_eq!(err.ustun, 2);
    assert!(!err.habar.is_empty())
}

#[wasm_bindgen_test]
fn test_kelgusi_validation_logic() {
    let router = Latinga::yangi(false); // Kelgusi

    // "shahar" is valid in Joriy but invalid in Kelgusi (should be "şahar")
    let input = "katta shahar";

    let js_val = router.tekshir(input, 10);
    let summary: TestSummary = serde_wasm_bindgen::from_value(js_val).unwrap();

    assert_eq!(summary.jami, 1);

    let err = &summary.hatolar[0];
    assert_eq!(err.soez, "shahar");
    assert_eq!(err.qator, 1);
    assert_eq!(err.ustun, 7);
}

#[wasm_bindgen_test]
fn test_validation_limit_behavior() {
    let router = Latinga::yangi(false);

    // Input has 3 errors: "sh", "sh", "sh"
    let input = "shahar shahar shahar";

    // Request max 2 errors
    let js_val = router.tekshir(input, 2);
    let summary: TestSummary = serde_wasm_bindgen::from_value(js_val).unwrap();

    // The Validator should count ALL of them...
    assert_eq!(summary.jami, 3);
    // ...but only return details for the first 2
    assert_eq!(summary.hatolar.len(), 2);
}

#[wasm_bindgen_test]
fn test_validation_success_case() {
    let router = Latinga::yangi(true);
    let input = "Maktab va Oʻquvchi"; // Correct Joriy spelling

    let js_val = router.tekshir(input, 10);
    let summary: TestSummary = serde_wasm_bindgen::from_value(js_val).unwrap();

    assert_eq!(summary.jami, 0);
    assert_eq!(summary.hatolar.len(), 0);
}
