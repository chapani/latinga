#![cfg(not(target_arch = "wasm32"))]

use latinga::Tartib;
mod common;

// --- Group 1: HTML & CSS Block Preservation ---

#[test]
fn test_shield_preserves_protected_tags_in_all_modes() {
    // Test all modes to ensure protection is a core engine feature, not mode-specific
    let modes = vec![Tartib::Joriy, Tartib::Kelgusi];

    let cases = vec![
        // 1. JavaScript strings & Logic
        r#"<script>$(".link").ShareLink({"title":"Бобур шаҳрида"});</script>"#,
        // 2. JSON-LD Metadata (keys and values)
        r#"<script type="application/ld+json">{"headline":"Бобур"}</script>"#,
        // 3. CSS Styles (selectors and content)
        r#"<style>.news::after { content: "Батафсил..."; }</style>"#,
        // 4. Preformatted blocks
        r#"<pre>Бу ерда матн ўзгармайди (Ғ)</pre>"#,
        // 5. Code tags
        r#"<code>const x = "Ғолиб";</code>"#,
    ];

    for mode in modes {
        let tr = common::setup_translator(mode, None);

        for input in &cases {
            assert_eq!(
                tr.oegir(input),
                *input,
                "Protection failed in {:?} mode for: {}",
                mode,
                input
            );
        }
    }
}

#[test]
fn test_shield_handles_web_entities_and_technical_scripts() {
    let joriy = common::setup_translator(Tartib::Joriy, None);
    let kelgusi = common::setup_translator(Tartib::Kelgusi, None);

    // 1. Entities should stay, but text should change
    // Joriy: Ғолиб -> Gʻolib
    assert_eq!(
        joriy.oegir("<div>&quot;Ғолиб&quot;</div>"),
        "<div>&quot;Gʻolib&quot;</div>"
    );

    // Kelgusi: Ғолиб -> Ğolib
    assert_eq!(
        kelgusi.oegir("<div>&quot;Ғолиб&quot;</div>"),
        "<div>&quot;Ğolib&quot;</div>"
    );

    // 2. Technical Code Blocks (Should stay exactly as input)
    let code_input = r#"<script>window.yaContextCb.push(()=>{});</script>"#;
    assert_eq!(joriy.oegir(code_input), code_input);
    assert_eq!(kelgusi.oegir(code_input), code_input);
}

// --- Group 2: LaTeX Structural & Syntax Protection ---

#[test]
fn test_latex_syntax_and_options_are_strictly_shielded() {
    let tr_kelgusi = common::setup_translator(Tartib::Kelgusi, None);

    // 1. Test Square Brackets (The 'english' -> 'engliş' bug)
    let pkg = r#"\usepackage[english]{babel}"#;
    assert_eq!(
        tr_kelgusi.oegir(pkg),
        pkg,
        "Failed to protect square bracket options!"
    );

    // 2. Test Key-Value pairs with 'sh' (should NOT become 'ş')
    let input = r#"\lstset{showstringspaces=false}"#;
    assert_eq!(
        tr_kelgusi.oegir(input),
        input,
        "Touched technical 'sh' in LaTeX key!"
    );

    // 3. Test Math Mode (should NOT change 'x' or 'y')
    let math = r#"$x = y + \Delta$"#;
    assert_eq!(tr_kelgusi.oegir(math), math, "Corrupted LaTeX math mode!");
}

// --- Group 3: Universal Shield Logic ---

#[test]
fn test_universal_shield_mechanics() {
    let tr = common::setup_translator(Tartib::Kelgusi, None);

    // 1. Standard Shield: Protects content, removes markers
    // 'shahar' would normally become 'şahar', but here it stays 'shahar'
    let standard = "Bu {]shahar[} markazi.";
    assert_eq!(tr.oegir(standard), "Bu shahar markazi.");

    // 2. Nested/Double Shield: Protects content, removes markers, KEEPS outer braces
    // Used when you need literal braces around protected text
    let nested = "Kod: {{]shahar[}}";
    assert_eq!(tr.oegir(nested), "Kod: {shahar}");
}

// --- Group 4: User-supplied Qalqons/Shields ---

#[test]
fn test_shield_handles_custom_config() {
    // This test covers:
    // 1. Parsing multi-line regex strings from Config.
    // 2. Handling Capture Groups (Shielding only the inner part).
    // 3. Handling Full Matches (Shielding the whole pattern).
    // 4. Verifying both Joriy and Kelgusi modes respect shields.

    let qalqons = r"
\*\*Русча:\*\* ([^\n]+)
\[oz\](.*?)\[/oz\]
";

    let joriy = common::setup_translator(Tartib::Joriy, Some(qalqons));
    let kelgusi = common::setup_translator(Tartib::Kelgusi, Some(qalqons));

    let input = "Атама\n**Русча:** Город\nБу [oz]Сақлаш[/oz] қилинади.";

    // Joriy: Ruscha -> Ruscha
    assert_eq!(
        joriy.oegir(input),
        "Atama\n**Ruscha:** Город\nBu [oz]Сақлаш[/oz] qilinadi."
    );

    // Kelgusi: Ruscha -> Rusça
    assert_eq!(
        kelgusi.oegir(input),
        "Atama\n**Rusça:** Город\nBu [oz]Сақлаш[/oz] qilinadi."
    );
}
