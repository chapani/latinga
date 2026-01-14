# Latinga 🇺🇿

Latinga is a high-performance, **Zero-Copy** Uzbek Cyrillic-Latin transliterator built in Rust. It is engineered for processing massive datasets and technical documents while preserving syntax integrity.

## Core Pipeline
Latinga follows a strict **Shield-Engine-Heal** workflow:
1. **Shielding**: Protects technical syntax (LaTeX, HTML, MD) and user-defined zones.
2. **Engine**: Performs high-speed conversion without unnecessary memory allocations.
3. **Healing**: Standardizes glottal stops, apostrophes, and handles proper noun logic.

## Key Features
- **Ultra-Fast**: Leverages `memmap2` and `Cow<'a, str>` for maximum throughput.
- **Context-Aware**: Automatically protects LaTeX math, HTML attributes, and code blocks.
- **Dual Orthography**: Supports **Current** (sh, ch, o') and **Proposed** (ş, ç, ö) standards.
- **Validation Mode**: Acts as a linter to find orthography errors in existing Latin texts.

## Installation
```
$ cargo install --path .
```

## Usage Examples
```
# Convert file to New Latin
$ latinga input.txt

# Convert via STDIN to Current Latin
$ echo "кирилл" | latinga -j

# Check file for orthography issues
$ latinga -s input.txt
```
