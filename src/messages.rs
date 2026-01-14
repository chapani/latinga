use crate::types::Tartib;
use std::borrow::Cow;

pub enum HabarKaliti {
    TekshiruvBoshi,
    TekshiruvHatosiNomi,
    TutuqUchunMaslahat,
    EskiQoeshHarf(String),
    QoeshHarfTutuqlaAjratilsin,
    QoeshimchaHatolar(usize),
    JarayonKetmoqda(String),
    JarayonHatosi(String, String),
    JarayonMuvaffaqiyati(usize),
    FaylTopilmadi,
}

impl HabarKaliti {
    pub fn koersat(&self, mode: &Tartib) -> Cow<'static, str> {
        match mode {
            Tartib::Joriy => match self {
                Self::TekshiruvBoshi => Cow::Borrowed("[!] Imlo xatolari aniqlandi"),
                Self::TekshiruvHatosiNomi => Cow::Borrowed("\x1b[1;31mxato\x1b[0m"),
                Self::TutuqUchunMaslahat => Cow::Borrowed(
                    "Maslahat: Avtomatik tuzatish uchun 'latinga' buyrugʻini ishlating.",
                ),
                Self::EskiQoeshHarf(s) => Cow::Owned(format!(
                    "Eski imlo belgisi aniqlandi. '{}' harfidan foydalaning.",
                    s
                )),
                Self::QoeshHarfTutuqlaAjratilsin => {
                    Cow::Borrowed("Shubhali 'sh/ch' birikmasi. Tutuq belgisi bilan ajrating.")
                }
                Self::QoeshimchaHatolar(n) => {
                    Cow::Owned(format!("  ... va yana {} ta xatolik.", n))
                }
                Self::JarayonKetmoqda(p) => Cow::Owned(format!("Oʻgirilmoqda: {p}")),
                Self::JarayonHatosi(p, e) => Cow::Owned(format!("Xatolik! {p}: {e}")),
                Self::JarayonMuvaffaqiyati(n) => {
                    Cow::Owned(format!("Muvaffaqiyatli yakunlandi: {n} ta fayl"))
                }
                Self::FaylTopilmadi => Cow::Borrowed("Xatolik: Fayl topilmadi."),
            },
            Tartib::Kelgusi => match self {
                Self::TekshiruvBoshi => Cow::Borrowed("[!] Imlo hatolari aniqlandi"),
                Self::TekshiruvHatosiNomi => Cow::Borrowed("\x1b[1;31mhato\x1b[0m"),
                Self::TutuqUchunMaslahat => {
                    Cow::Borrowed("Maslahat: Avtomatik tuzatiş uchun 'latinga' buyruğini işlating.")
                }
                Self::EskiQoeshHarf(s) => Cow::Owned(format!(
                    "Eski imlo belgisi aniqlandi. '{}' harfidan foydalaning.",
                    s
                )),
                Self::QoeshHarfTutuqlaAjratilsin => {
                    Cow::Borrowed("Şubhali 'sh/ch' birikmasi. Tutuq belgisi bilan ajrating.")
                }
                Self::QoeshimchaHatolar(n) => {
                    Cow::Owned(format!("  ... va yana {} ta hatolik.", n))
                }
                Self::JarayonKetmoqda(p) => Cow::Owned(format!("Ögirilmoqda: {p}")),
                Self::JarayonHatosi(p, e) => Cow::Owned(format!("Hatolik! {p}: {e}")),
                Self::JarayonMuvaffaqiyati(n) => {
                    Cow::Owned(format!("Muvaffaqiyatli yakunlandi: {n} ta fayl"))
                }
                Self::FaylTopilmadi => Cow::Borrowed("Hatolik: Fayl topilmadi."),
            },
        }
    }
}
