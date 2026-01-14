use crate::{Oegirgich, Tartib};
use serde::Serialize;
use wasm_bindgen::prelude::*;

// 1. Define Serializable Structs for JS
#[derive(Serialize)]
pub struct WasmTekshiruvHatosi {
    pub qator: usize,
    pub ustun: usize,
    pub soez: String,
    pub habar: String,
}

#[derive(Serialize)]
pub struct WasmTekshiruvHulosasi {
    pub hatolar: Vec<WasmTekshiruvHatosi>,
    pub jami: usize,
}

#[wasm_bindgen]
pub struct Latinga {
    ichki: Oegirgich,
}

#[wasm_bindgen]
impl Latinga {
    #[wasm_bindgen(constructor)]
    pub fn yangi(is_joriy: bool) -> Self {
        let mode = if is_joriy {
            Tartib::Joriy
        } else {
            Tartib::Kelgusi
        };
        Self {
            ichki: Oegirgich::fitrat_ila_yangi(mode),
        }
    }

    pub fn almashuvchilarni_yukla(&mut self, rules: &str) {
        let clean_rules = rules.replace(';', "\n");
        self.ichki.sozlama.almashuvchilarni_yukla(&clean_rules);
    }

    pub fn atoqlilarni_yukla(&mut self, list: &str) {
        let clean_list = list.replace(',', "\n");
        self.ichki.sozlama.atoqlilarni_yukla(&clean_list);
    }

    pub fn qalqonlarni_yukla(&mut self, pattern: &str) -> bool {
        match self.ichki.sozlama.qalqonlarni_yukla(pattern) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn oegir(&self, input: &str) -> String {
        self.ichki.oegir(input)
    }

    /// Returns a JsValue (JSON Object) containing the validation summary.
    pub fn tekshir(&self, input: &str, limit: usize) -> JsValue {
        let summary = self.ichki.tekshir(input, limit);

        // Map Zero-Copy structs to Owned Serializable structs
        let wasm_summary = WasmTekshiruvHulosasi {
            jami: summary.jami,
            hatolar: summary
                .hatolar
                .into_iter()
                .map(|e| WasmTekshiruvHatosi {
                    qator: e.qator,
                    ustun: e.ustun,
                    soez: e.soez.to_string(),
                    habar: e.habar.to_string(),
                })
                .collect(),
        };

        // Serialize to JS Object
        serde_wasm_bindgen::to_value(&wasm_summary).unwrap()
    }
}
