use crate::engine::Engine;
use crate::validator::Validator;
use crate::{HabarKaliti, Sozlama, Tartib, TekshiruvHulosasi};
use std::borrow::Cow;
use std::io;

/// The Facade for the Latinga library.
/// Orchestrates configuration, engine execution, and validation.
pub struct Oegirgich {
    pub sozlama: Sozlama,
}

impl Oegirgich {
    #[must_use]
    pub fn yangi(config: Sozlama) -> Self {
        Self { sozlama: config }
    }

    #[must_use]
    pub fn fitrat_ila_yangi(mode: Tartib) -> Self {
        let mut config = Sozlama::yangi(mode);
        config.lughat.load_defaults(mode);
        Self { sozlama: config }
    }

    #[must_use]
    pub fn habar(&self, key: HabarKaliti) -> Cow<'static, str> {
        key.koersat(&self.sozlama.tartib)
    }

    #[must_use]
    pub fn oegir(&self, input: &str) -> String {
        if input.is_empty() {
            return String::new();
        }
        let engine = Engine::new(&self.sozlama);
        engine.run(input)
    }

    pub fn oqimni_oegir<W: io::Write + ?Sized>(
        &self,
        input: &str,
        writer: &mut W,
    ) -> io::Result<()> {
        if input.is_empty() {
            return Ok(());
        }
        let engine = Engine::new(&self.sozlama);
        engine.convert_stream(input, writer)
    }

    #[must_use]
    pub fn tekshir<'a>(&self, input: &'a str, limit: usize) -> TekshiruvHulosasi<'a> {
        if input.is_empty() {
            return TekshiruvHulosasi {
                hatolar: vec![],
                jami: 0,
            };
        }

        let validator = Validator::new(&self.sozlama);
        validator.check_errors(input, limit)
    }
}
