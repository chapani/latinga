#![cfg(not(target_arch = "wasm32"))]

use assert_cmd::Command;
use assert_cmd::cargo_bin;
use latinga::{HabarKaliti, OKINA, TUTUQ, Tartib};
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

// --- GROUP 1: METADATA & FLAG VALIDATION ---

#[test]
fn test_help_flag_displays_usage_information() {
    let mut cmd = Command::new(cargo_bin!("latinga"));
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE"));
}

#[test]
fn test_invalid_flag_returns_exit_failure() {
    let mut cmd = Command::new(cargo_bin!("latinga"));
    cmd.arg("--unknown-flag").assert().failure();
}

#[test]
fn test_missing_file_returns_error_instead_of_hanging() {
    let mut cmd = Command::new(cargo_bin!("latinga"));
    cmd.arg("bu_fayl_mavjud_emas.txt").assert().failure();
}

// --- GROUP 2: STANDARD INPUT (PIPING) ---

#[test]
fn test_stdin_converts_to_kelgusi_by_default() {
    let mut cmd = Command::new(cargo_bin!("latinga"));
    // Default is Kelgusi: 'sh' -> 'ş', 'ch' -> 'ç'
    cmd.write_stdin("shahar choy")
        .assert()
        .success()
        .stdout(predicate::str::contains("şahar çoy"));
}

#[test]
fn test_stdin_joriy_mode_standardizes_vowels() {
    let mut cmd = Command::new(cargo_bin!("latinga"));
    // Using -j for joriy mode.
    // We expect the official O_OKINA constant character.
    let expected = format!("o{OKINA}rdak");

    cmd.arg("-j")
        .write_stdin("ўрдак")
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[test]
fn test_stdin_joriy_mode_standardizes_glottal_stops() {
    let mut cmd = Command::new(cargo_bin!("latinga"));
    // Testing Arabic-origin word with a glottal stop
    let expected = format!("Ma{TUTUQ}no");

    cmd.arg("-j")
        .write_stdin("Маъно")
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

// --- GROUP 3: FILE SYSTEM OPERATIONS ---

#[test]
fn test_file_output_generates_correct_default_suffix() {
    let dir = tempdir().unwrap();
    let input_file = dir.path().join("test.txt");

    // Expect -kelgusiga because joriy is false by default
    let expected_output = dir.path().join("test-kelgusiga.txt");

    fs::write(&input_file, "кирилл").unwrap();

    let mut cmd = Command::new(cargo_bin!("latinga"));
    cmd.arg(&input_file).assert().success();

    let result = fs::read_to_string(expected_output)
        .expect("Fayl topilmadi: test-kelgusiga.txt kutilgan edi");

    assert_eq!(result.trim(), "kirill");
}

#[test]
fn test_file_output_respects_custom_suffix_and_mode_flags() {
    let dir = tempdir().unwrap();
    let input_file = dir.path().join("test.txt");
    fs::write(&input_file, "кирилл").unwrap();

    let mut cmd = Command::new(cargo_bin!("latinga"));

    cmd.arg(&input_file)
        .arg("--joriy")
        .arg("--chiqarma=-converted");

    cmd.assert().success();

    let output_path = dir.path().join("test-converted.txt");
    assert!(output_path.exists());

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains("kirill"));
}

#[test]
fn test_file_inplace_modification_overwrites_original() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("inplace.txt");
    // Testing 'Объект' -> 'Obyekt' (dropping the hard sign linguistically)
    fs::write(&file_path, "Объект").unwrap();

    let mut cmd = Command::new(cargo_bin!("latinga"));
    cmd.arg(&file_path).arg("--ustidan-yoz").assert().success();

    let result = fs::read_to_string(file_path).unwrap();
    assert_eq!(result.trim(), "Obyekt");
}

#[test]
fn test_glob_pattern_processes_multiple_files_matching_criteria() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    fs::create_dir(&sub).unwrap();

    let file1 = sub.join("1.txt");
    let file2 = sub.join("2.txt");
    fs::write(&file1, "о").unwrap(); // Cyrillic 'o'
    fs::write(&file2, "а").unwrap(); // Cyrillic 'a'

    let mut cmd = Command::new(cargo_bin!("latinga"));
    let pattern = format!("{}/sub/*.txt", dir.path().to_str().unwrap());

    cmd.arg("--fayl-qolip")
        .arg(pattern)
        .arg("--joriy")
        .arg("--ustidan-yoz");

    cmd.assert().success();

    let content1 = fs::read_to_string(file1).unwrap();
    let content2 = fs::read_to_string(file2).unwrap();

    assert_eq!(content1, "o");
    assert_eq!(content2, "a");
}

// --- GROUP 4: SHIELDING & SPECIAL FORMATS ---

#[test]
fn test_qalqon_shielding_prevents_translation_of_specified_terms() {
    let mut cmd = Command::new(cargo_bin!("latinga"));
    cmd.arg("--qalqon")
        .arg("Xerox")
        .write_stdin("Xerox компанияси")
        .assert()
        .success()
        .stdout(predicate::str::contains("Xerox kompaniyasi"));
}

#[test]
fn test_latex_shielding_preserves_comments_and_commands() {
    let dir = tempdir().unwrap();
    let tex_file = dir.path().join("paper.tex");

    let input = r"\section{Кириш}
% Бу изоҳ (comment)
Бу ерда shahar бор. \cite{shahar2024}";

    fs::write(&tex_file, input).unwrap();

    let mut cmd = Command::new(cargo_bin!("latinga"));
    cmd.arg(&tex_file).arg("--joriy");

    cmd.assert().success();

    let out_path = dir.path().join("paper-joriyga.tex");
    let output =
        fs::read_to_string(out_path).expect("Output file was not found with -joriyga suffix");

    assert!(output.contains(r"\section{Kirish}"));
    assert!(output.contains(r"% Бу изоҳ (comment)"));
    assert!(output.contains(r"\cite{shahar2024}"));
    assert!(output.contains("shahar bor"));
}

// --- GROUP 5: VALIDATION & DIAGNOSTICS (-s, -t) ---

#[test]
fn test_validation_mode_detects_errors_and_fails() {
    let mut cmd = Command::new(cargo_bin!("latinga"));

    // We provide text that definitely has validation errors
    // (e.g., using 'sh' instead of 'ş' in Kelgusi mode, or soft signs).
    // Assuming standard behavior triggers warnings for common mistakes.
    cmd.arg("--tekshir") // Validation mode
        .write_stdin("shahar") // 'sh' should be 'ş' in Kelgusi validation
        .assert()
        .failure(); // Should exit with status 1 if errors found
}

#[test]
fn test_validation_clean_input_returns_success() {
    let mut cmd = Command::new(cargo_bin!("latinga"));

    // Valid input for Kelgusi mode
    cmd.arg("--tekshir").write_stdin("şahar").assert().success(); // Exit status 0
}

#[test]
fn test_verbose_mode_prints_status_messages() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("verbose.txt");
    fs::write(&file_path, "test").unwrap();

    let mut cmd = Command::new(cargo_bin!("latinga"));

    cmd.arg(&file_path)
        .arg("--batafsil")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            HabarKaliti::JarayonMuvaffaqiyati(1).koersat(&Tartib::Kelgusi),
        )); // "Processing" message
}

#[test]
fn test_uzbek_help_flag_prints_custom_help() {
    let mut cmd = Command::new(cargo_bin!("latinga"));

    cmd.arg("--yordam") // Yordam
        .assert()
        .success()
        .stdout(predicate::str::contains("FOYDALANIŞ")); // Uzbek usage header
}

// --- GROUP 6: CUSTOM DICTIONARIES & CONFIG (-a, -i, -f) ---

#[test]
fn test_custom_proper_nouns_with_apostrophes() {
    let mut cmd = Command::new(cargo_bin!("latinga"));

    // -a "windows" -> should be treated as proper noun "windows" -> "Windows"
    cmd.arg("--atoqli")
        .arg("Samarqand,Ğijduvon,Özbekiston")
        .write_stdin("O'zbekistonda Samarqandga G'ijduvondan borilmaydi.")
        .assert()
        .success()
        .stdout(predicate::eq(
            "Özbekiston'da Samarqand'ga Ğijduvon'dan borilmaydi.",
        ));
}

#[test]
fn test_custom_proper_nouns_with_sh_and_ch() {
    let mut cmd = Command::new(cargo_bin!("latinga"));

    // -a "windows" -> should be treated as proper noun "windows" -> "Windows"
    cmd.arg("--atoqli")
        .arg("Şaşmaqom,Çirçiq")
        .write_stdin("Shashmaqomda va Chirchiqda baliq bor.")
        .assert()
        .success()
        .stdout(predicate::eq("Şaşmaqom'da va Çirçiq'da baliq bor."));
}

#[test]
fn test_substitutes_file_overrides_standard_rules() {
    let dir = tempdir().unwrap();
    let exc_file = dir.path().join("my_substitutes.txt");
    fs::write(&exc_file, "октябрь : oktabr").unwrap();

    let mut cmd = Command::new(cargo_bin!("latinga"));

    cmd.arg("--almashtir")
        .arg(exc_file)
        .write_stdin("Октябрь кириб келди.")
        .assert()
        .success()
        .stdout(predicate::eq("Oktabr kirib keldi."));
}

#[test]
fn test_shield_file_prevents_translation() {
    let dir = tempdir().unwrap();
    let shield_file = dir.path().join("safe.txt");
    fs::write(&shield_file, "Бола").unwrap();

    let mut cmd = Command::new(cargo_bin!("latinga"));

    cmd.arg("--qalqon-fayl")
        .arg(shield_file)
        .write_stdin("Бола чопти.")
        .assert()
        .success()
        .stdout(predicate::str::contains("Бола"));
}

#[test]
fn test_multiple_shield_flags_work_together() {
    let mut cmd = Command::new(cargo_bin!("latinga"));

    cmd.arg("-q")
        .arg("Биринчи")
        .arg("-q")
        .arg("Иккинчи")
        .write_stdin("Биринчи ва Иккинчи")
        .assert()
        .success()
        .stdout(predicate::str::contains("Биринчи va Иккинчи"));
}
