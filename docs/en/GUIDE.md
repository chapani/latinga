# Latinga: Technical User Guide

Latinga is command-line transliterator for Uzbek latin text. This guide explains how to use its advanced features.

## 1. Orthography Modes

Latinga supports two primary standards:

| Mode | Flag | Example (Cyrillic -> Latin) |
| :--- | :--- | :--- |
| **New Latin** (Proposed) | Default | шаҳар -> şahar, ўрдак -> ördak |
| **Current Latin** | -j, --joriy | шаҳар -> shahar, ўрдак -> oʻrdak |

## 2. Proper Nouns & Suffixes (-a, --atoqli)
In the New Latin (Kelgusi) standard, suffixes attached to proper nouns should be separated by an apostrophe. Latinga automates this:

```
$ latinga -a "London,Paris"
# Input: "Londonga boramiz."
# Output: "London'ga boramiz."
```

## 3. Protecting Content (Shielding)

To ensure technical strings are not corrupted during conversion, Latinga provides multiple shielding layers:

### A. Automatic Shielding

The following are protected out of the box:

- **LaTeX**: Blocks like `$f(x) = y$` and commands like `\cite{...}` are skipped.
- **HTML**: Tags are protected, while specific attributes like `content` in meta tags are surgically unmasked for conversion.
- **Emails/URLs**: `info@latinga.uz` or `https://...` remain untouched.

### B. Universal Shield `{] ... [}`

Wrap any text in these markers to prevent conversion:

- Input: `Cyrillic {]Ер[} stays Cyrillic.`
- Output: `Cyrillic Ер stays Cyrillic.`

### C. Regex Shielding (-n, --qalqon)

Define custom patterns to protect specific text segments:
```
$ latinga input.txt -n "\[ID:[0-9]+\]"
```

## 4. Custom Mappings (-m, --almashtir)

Override standard rules or add project-specific substitutions:

```
# Provide rules as a string
$ latinga input.txt -m "октябрь=>oktabr;сентябрь=>sentabr"

# Or provide a rule file
$ latinga input.txt -m rules.txt
```

## 5. Validation Mode (-s, --sinab-kor)

Use Latinga as a linter to find errors in Latin text (e.g., finding sh where ş is expected in Kelgusi mode):

```
$ latinga -s input.txt
```

If errors are found, it outputs the line/column and exits with status 1, making it ideal for CI/CD pipelines.
