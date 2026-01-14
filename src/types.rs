use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TekshiruvHatosi<'a> {
    pub qator: usize,
    pub ustun: usize,
    /// Reference to the specific word in the source text (Zero-Copy)
    pub soez: Cow<'a, str>,
    /// Message can be a static string literal or a formatted string
    pub habar: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TekshiruvHulosasi<'a> {
    #[serde(borrow)]
    pub hatolar: Vec<TekshiruvHatosi<'a>>,
    pub jami: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tartib {
    Joriy,
    Kelgusi,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Chunk<'a> {
    Safe(&'a str),
    Shielded(&'a str),
}
