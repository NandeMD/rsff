/// Supported output file types.
/// 
/// `RAW`: Raw XML string
/// `ZLIB`: Compressed XML
/// `TXT`: Raw, lossy .txt file
#[derive(Clone)]
pub enum OUT {
    RAW,
    ZLIB,
    TXT,
}

/// Balloon types. Default value is `DIALOGUE`.
/// 
/// ST: Sub-text\
/// OT: Over-text
#[derive(PartialEq, Debug, Clone)]
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