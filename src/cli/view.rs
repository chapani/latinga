#![cfg(feature = "cli")]

use latinga::TekshiruvHatosi;
use std::path::Path;

// --- 1. Terminal Rendering Logic ---

pub fn render_error(path: &Path, full_text: &str, err: &TekshiruvHatosi, label: &str) {
    // Locate the specific line for context
    let original_line = full_text.lines().nth(err.qator - 1).unwrap_or("");
    let line_num = err.qator.to_string();
    let gutter_width = line_num.len();

    // Header: Label and location
    eprintln!(
        "{}: {} ({}:{}:{})",
        label,
        err.habar,
        path.display(),
        err.qator,
        err.ustun
    );

    // Code snippet visualization
    eprintln!("{:>width$} |", "", width = gutter_width);
    eprintln!("{} | {}", line_num, original_line);

    // Caret pointing to error column
    // +3 accounts for " | " structure
    let padding = gutter_width + 3 + (err.ustun.saturating_sub(1));
    eprintln!("{}\x1b[1;32m^\x1b[0m", " ".repeat(padding));
    eprintln!("{:>width$} |", "", width = gutter_width);
}

// --- 2. Static Help Content ---

pub fn print_uz_help() {
    let version = env!("CARGO_PKG_VERSION");
    println!(
        r#"Latinga v{version} - Özbek Lotin Alifbosi Ögiruvçisi

FOYDALANIŞ: latinga [FAYLLAR] [BAYROQLAR]

BAYROQLAR:
  -j, --joriy          Joriy imlo (sh, ch, oʻ, gʻ, x, h) - Fitriy holat: Yangi imlo (ş, ç, ö, ğ, h)
  -u, --ustidan-yoz    Fayllarni öz joyida özgartiriş (ehtiyot böling!)
  -f, --fayl-qolip     Fayl qolipi (masalan: "kitoblar/*.txt")
  -c, --chiqarma       Yangi fayl nomi qöşimçasi (fitrat: "-joriyga" yoki "-kelgusiga")
  -m, --almashtir      Almaştiruvlar luğati (txt fayl yölagi yoki 'eski:yangi;eski2:yangi2')
  -a, --atoqli         Atoqli otlar luğati (tutuq belgisi bilan ajratilişi uçun)
  -q, --qalqon         Ifodali himoya qoliplari (ögirilmaydigan qismlar uçun)
  -n, --qalqon-fayl    Ifodali himoya qoliplari fayli yölagi
  -b, --batafsil       Bajarilgan işlar tafsilotini körsatiş
  -t, --tekshir        Imlo va qoidalarni tekşiriş (ihtiyoriy: körsatiladigan hatolar soni)
  -y, --yordam         Özbekça yordam (uşbu ekran)
  -h, --help           English help

MAHSUS IMKONIYATLAR:
  Umumiy Himoya:       Matn içidagi {{]himoyalangan[}} qismlar özgarmaydi.
  Aqlli Qöştirnoq:     'sözlar' dagi qöştirnoqlar saqlanadi, söz içidagi tutuq belgilari
                       esa avtomatik tartibga solinadi.

MISOLLAR:
  latinga matn.txt                     # Yangi imloga ögiriş
  latinga matn.txt --joriy             # Joriy imloga ögiriş
  latinga *.md -u                      # Barça Markdown fayllarni öz joyida özgartiriş"#
    );
}

pub fn print_en_help() {
    let version = env!("CARGO_PKG_VERSION");
    println!(
        r#"Latinga v{version} - Uzbek Latin Alphabet Transliterator

USAGE: latinga [FILES] [FLAGS]

FLAGS:
  -j, --joriy          Use current orthography (sh, ch, oʻ, gʻ, x, h) - Default: Future (ş, ç, ö, ğ, h)
  -u, --ustidan-yoz    Overwrite files in-place
  -f, --fayl-qolip     Input glob pattern (e.g. "docs/*.md")
  -c, --chiqarma       Output filename suffix (default: "-joriyga" or "-kelgusiga")
  -m, --almashtir      Custom substitutions dictionary path or 'key:value;key2:value2'
  -a, --atoqli         Proper nouns dictionary path
  -q, --qalqon         Regex protection pattern
  -n, --qalqon-fayl    File containing regex protection patterns
  -b, --batafsil       Verbose details mode
  -t, --tekshir        Text validation (Optional: number of detailed errors)
  -y, --yordam         Uzbek help
  -h, --help           English help (this screen)

SPECIAL FEATURES:
  Universal Shield:    Text inside {{]protected[}} markers will not be converted.
  Smart Quotes:        Preserves 'quoted' phrases while standardizing internal
                       apostrophes (e.g., ma'no -> maʼno).

EXAMPLES:
  latinga input.txt                    # Convert to New Proposed Latin
  latinga input.txt --joriy            # Convert to Current Latin
  latinga "docs/*.txt" -c "-fixed"     # Batch convert with custom suffix"#
    );
}
