pub enum OUT {
    RAW,
    ZLIB,
    TXT,
}

#[derive(PartialEq, Debug)]
pub enum TYPES {
    DIALOGUE,
    SQUARE,
    THINKING,
    ST,
    OT
}

impl Default for TYPES {
    fn default() -> Self {
        Self::DIALOGUE
    }
}