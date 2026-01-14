use crate::Tartib;
use crate::dictionary::Dictionary;

pub struct Sozlama {
    pub tartib: Tartib,
    pub lughat: Dictionary,
}

impl Sozlama {
    #[must_use]
    pub fn yangi(tartib: Tartib) -> Self {
        let mut lughat = Dictionary::new();
        Self::setup_dictionary(&mut lughat, tartib);
        Self { tartib, lughat }
    }

    /// Private helper to orchestrate the loading of all embedded assets
    fn setup_dictionary(dict: &mut Dictionary, mode: Tartib) {
        // 2. Load all other default dictionary assets
        dict.load_defaults(mode);
    }

    pub fn qalqonlarni_yukla(&mut self, c: &str) -> Result<(), regex::Error> {
        self.lughat.load_qalqons(c)
    }
    pub fn atoqlilarni_yukla(&mut self, c: &str) {
        self.lughat.load_proper_nouns(c);
    }
    pub fn qoeshimchalarni_yukla(&mut self, c: &str) {
        self.lughat.load_suffixes(c);
    }
    pub fn almashuvchilarni_yukla(&mut self, c: &str) {
        self.lughat.load_substitutions(c);
    }
    pub fn tuzatishlarni_yukla(&mut self, c: &str) {
        self.lughat.load_healing(c);
    }
}
