use anyhow::Result;
use clap::Parser;
use latinga::{HabarKaliti, Oegirgich, Sozlama, Tartib};
use memmap2::MmapOptions;
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

mod cli;
use cli::{files, view};

/// Default number of detailed errors to show in validation mode
const DEFAULT_VALIDATION_LIMIT: usize = 5;

#[derive(Parser)]
#[command(name = "latinga", version, disable_help_flag = true)]
struct Cli {
    /// Files to process
    files: Vec<PathBuf>,

    #[arg(short = 'j', long = "joriy")]
    joriy: bool,

    #[arg(short = 'u', long = "ustidan-yoz")]
    ustidan_yoz: bool,

    #[arg(short = 'f', long = "fayl-qolip")]
    fayl_qolipi: Option<String>,

    #[arg(short = 'a', long = "atoqli")]
    atoqli: Option<String>,

    #[arg(short = 'c', long = "chiqarma", allow_hyphen_values = true)]
    chiqarma_qolipi: Option<String>,

    #[arg(short = 'q', long = "qalqon", action = clap::ArgAction::Append)]
    qalqon: Vec<String>,

    #[arg(short = 'n', long = "qalqon-fayl")]
    qalqon_fayl: Option<PathBuf>,

    #[arg(short = 'b', long = "batafsil")]
    batafsil: bool,

    #[arg(short = 'm', long = "almashtir")]
    almashtir: Option<String>,

    #[arg(short = 't', long = "tekshir")]
    tekshir: Option<Option<usize>>,

    #[arg(short = 'y', long = "yordam")]
    yordam: bool,

    #[arg(short = 'h', long = "help")]
    help: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mode = if cli.joriy {
        Tartib::Joriy
    } else {
        Tartib::Kelgusi
    };

    // 1. Handle Meta-commands
    if cli.yordam {
        view::print_uz_help();
        return Ok(());
    }
    if cli.help {
        view::print_en_help();
        return Ok(());
    }

    // 2. Setup Suffix and Sanitization
    let mut current_suffix = cli.chiqarma_qolipi.clone().unwrap_or_else(|| match mode {
        Tartib::Joriy => "-joriyga".to_string(),
        _ => "-kelgusiga".to_string(),
    });
    files::sanitize_string(&mut current_suffix);

    // 3. Build Configuration and Discover Files
    let config = build_config(&cli)?;
    let translator = Oegirgich::yangi(config);
    let targets = files::discover_files(&cli.files, cli.fayl_qolipi.as_deref())?;

    if targets.is_empty() && (!cli.files.is_empty() || cli.fayl_qolipi.is_some()) {
        eprintln!("{}", HabarKaliti::FaylTopilmadi.koersat(&mode));
        std::process::exit(1);
    }

    // 4. Dispatch to Workflows
    let validation_limit = match cli.tekshir {
        None => None,
        Some(None) => Some(DEFAULT_VALIDATION_LIMIT),
        Some(Some(n)) => Some(n),
    };

    if let Some(error_limit) = validation_limit {
        let has_errors = if targets.is_empty() {
            validate_stdin(&translator, error_limit)?
        } else {
            validate_files(&targets, &translator, error_limit)?
        };

        if has_errors {
            std::process::exit(1);
        }
    } else if targets.is_empty() {
        process_stdin(&translator)?;
    } else {
        process_files(&targets, &translator, &cli, &current_suffix)?;
    }

    Ok(())
}

fn build_config(cli: &Cli) -> Result<Sozlama> {
    let mode = if cli.joriy {
        Tartib::Joriy
    } else {
        Tartib::Kelgusi
    };
    let mut cfg = Sozlama::yangi(mode);

    if let Some(raw_input) = &cli.almashtir {
        // Resolve input: File Path OR Semicolon-delimited String
        let content = resolve_input_source(raw_input, ';')?;
        cfg.almashuvchilarni_yukla(&content);
    }

    // Versatile Proper Noun Loading (Already existed, just kept clean)
    if let Some(raw_input) = &cli.atoqli {
        match mode {
            Tartib::Kelgusi => {
                // Resolve input: File Path OR Comma-delimited String
                let content = resolve_input_source(raw_input, ',')?;
                cfg.atoqlilarni_yukla(&content);
            }
            Tartib::Joriy => {
                eprintln!("Diqqat: -a, --atoqli bayrogʻi faqat Kelgusi tartibida ishlaydi.");
            }
        }
    }

    for n in &cli.qalqon {
        cfg.qalqonlarni_yukla(n)?;
    }
    if let Some(p) = &cli.qalqon_fayl {
        cfg.qalqonlarni_yukla(&fs::read_to_string(p)?)?;
    }

    Ok(cfg)
}

/// Helper to handle the "File vs String" logic.
/// - If `input` is a valid file path, returns the file content.
/// - Otherwise, returns the input string, replacing the `delimiter` with newlines
///   so the dictionary loader can process it.
fn resolve_input_source(input: &str, delimiter: char) -> Result<String> {
    let path = Path::new(input);
    // Check if it exists specifically as a file to avoid confusion
    if path.exists() && path.is_file() {
        Ok(fs::read_to_string(path)?)
    } else {
        Ok(input.replace(delimiter, "\n"))
    }
}

fn process_stdin(trans: &Oegirgich) -> Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    if !buf.is_empty() {
        // STREAMING: Write directly to stdout lock via BufWriter for performance
        let stdout = io::stdout();
        let mut writer = io::BufWriter::new(stdout.lock());
        trans.oqimni_oegir(&buf, &mut writer)?;
        writer.flush()?;
    }
    Ok(())
}

fn process_files(
    files: &BTreeSet<PathBuf>,
    trans: &Oegirgich,
    cli: &Cli,
    suffix: &str,
) -> Result<()> {
    use rayon::prelude::*;

    // Parallel processing for file conversion
    let successful_count: usize = files
        .par_iter()
        .map(|p| process_single_file(p, trans, cli, suffix))
        .sum();

    if cli.batafsil {
        println!(
            "{}",
            trans.habar(HabarKaliti::JarayonMuvaffaqiyati(successful_count))
        );
    }
    Ok(())
}

fn process_single_file(p: &PathBuf, trans: &Oegirgich, cli: &Cli, suffix: &str) -> usize {
    let p_str = p.to_string_lossy();

    if cli.batafsil {
        println!(
            "{}",
            trans.habar(HabarKaliti::JarayonKetmoqda(p_str.to_string()))
        );
    }

    // Use IIFE to handle Result logic concisely
    let result = (|| -> Result<()> {
        let file = File::open(p)?;

        // Zero-Copy: Map file to memory
        // SAFETY: We assume the file is not modified externally during processing
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let content = std::str::from_utf8(&mmap)?;

        if cli.ustidan_yoz {
            // STREAMING: Atomic write with a streaming closure
            files::atomic_write_stream(p, |writer| trans.oqimni_oegir(content, writer))?;
        } else {
            // STREAMING: Standard file write via BufWriter
            let out_path = files::get_output_path(p, suffix);
            let out_file = File::create(out_path)?;
            let mut writer = io::BufWriter::new(out_file);
            trans.oqimni_oegir(content, &mut writer)?;
            writer.flush()?;
        }
        Ok(())
    })();

    match result {
        Ok(_) => 1,
        Err(e) => {
            eprintln!(
                "{}",
                trans.habar(HabarKaliti::JarayonHatosi(p_str.to_string(), e.to_string()))
            );
            0
        }
    }
}

fn validate_files(files: &BTreeSet<PathBuf>, trans: &Oegirgich, limit: usize) -> Result<bool> {
    use rayon::prelude::*;

    // Parallel processing for validation
    let global_failure = files
        .par_iter()
        .map(|p| validate_single_file(p, trans, limit))
        .any(|has_err| has_err);

    Ok(global_failure)
}

fn validate_single_file(p: &PathBuf, trans: &Oegirgich, limit: usize) -> bool {
    // Fail gracefully if file IO fails
    let Ok(file) = File::open(p) else {
        return false;
    };
    // SAFETY: Memory mapping file
    let Ok(mmap) = (unsafe { MmapOptions::new().map(&file) }) else {
        return false;
    };
    let Ok(content) = std::str::from_utf8(&mmap) else {
        return false;
    };

    // Zero-Copy: 'summary' holds references to 'content' (the mmap)
    let summary = trans.tekshir(content, limit);

    if !summary.hatolar.is_empty() {
        eprintln!(
            "\n{}: {}",
            trans.habar(HabarKaliti::TekshiruvBoshi),
            p.display()
        );

        let label = trans.habar(HabarKaliti::TekshiruvHatosiNomi);
        for err in summary.hatolar.iter().take(limit) {
            view::render_error(p, content, err, &label);
        }

        if summary.jami > limit {
            eprintln!(
                "{}",
                trans.habar(HabarKaliti::QoeshimchaHatolar(summary.jami - limit))
            );
        }
        eprintln!("{}", trans.habar(HabarKaliti::TutuqUchunMaslahat));

        return true;
    }

    false
}

fn validate_stdin(trans: &Oegirgich, limit: usize) -> Result<bool> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    // Zero-Copy: 'summary' holds references to 'buf'
    let summary = trans.tekshir(&buf, limit);

    if !summary.hatolar.is_empty() {
        eprintln!("\n{}: stdin", trans.habar(HabarKaliti::TekshiruvBoshi));
        let label = trans.habar(HabarKaliti::TekshiruvHatosiNomi);

        for err in summary.hatolar.iter().take(limit) {
            view::render_error(Path::new("stdin"), &buf, err, &label);
        }

        if summary.jami > limit {
            eprintln!(
                "{}",
                trans.habar(HabarKaliti::QoeshimchaHatolar(summary.jami - limit))
            );
        }

        eprintln!("{}", trans.habar(HabarKaliti::TutuqUchunMaslahat));
        return Ok(true);
    }
    Ok(false)
}
