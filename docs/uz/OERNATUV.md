# Oʻrnatish Qoʻllanmasi

**Latinga** dasturidan tayyor binar fayllarni yuklab olish yoki manba kodidan yigʻish orqali foydalanishingiz mumkin.

## 1. Tayyor binar fayllarni yuklab olish
[Releases](https://github.com/yourusername/latinga/releases) sahifasiga oʻting va operatsion tizimingizga mos faylni tanlang:

### 🐧 Linux
1. `latinga-linux-amd64` faylini yuklab oling.
2. Unga ruxsat bering: `chmod +x latinga-linux-amd64`.
3. Tizimga joylang: `sudo mv latinga-linux-amd64 /usr/local/bin/latinga`.

### 🍎 macOS (Apple Silicon M1/M2/M3)
macOS xavfsizlik choralari tufayli quyidagi amallarni bajaring:
1. `latinga-macos-arm64` faylini yuklab oling.
2. Terminalda buyruqni bering: `chmod +x latinga-macos-arm64`.
3. Faylni Finder orqali toping, **oʻng tugmani** bosing va **Open** tanlang.
4. Ogohlantirish chiqsa, **Open** tugmasini bosing (bu faqat bir marta soʻraladi).
5. Tizimga joylang: `sudo mv latinga-macos-arm64 /usr/local/bin/latinga`.

### 🪟 Windows
1. `latinga-windows-amd64.exe` faylini yuklab oling.
2. (Ixtiyoriy) Nomini `latinga.exe` ga oʻzgartirib, tizim PATH qismiga qoʻshishingiz mumkin.

## 2. Cargo orqali oʻrnatish
```bash
cargo install latinga --features cli
