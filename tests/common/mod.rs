use latinga::{Oegirgich, Tartib};

pub fn setup_translator(mode: Tartib, qalqon: Option<&str>) -> Oegirgich {
    let mut translator = Oegirgich::fitrat_ila_yangi(mode);

    if let Some(pattern) = qalqon {
        // Access .config directly
        translator
            .sozlama
            .qalqonlarni_yukla(pattern)
            .expect("Invalid Regex in test setup");
    }

    translator
}
