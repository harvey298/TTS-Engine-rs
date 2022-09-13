// For Voice Managment

/// Tikoks TTS engine wrapper
#[cfg(feature = "tiktok")]
pub mod tiktok;

/// Streamlabs Polly TTS engine wrapper
#[cfg(feature = "streamlabs")]
pub mod streamlabs;

/// A List of TTS Engines currently supported
#[allow(unused)]
pub const TTS_ENGINES: [&str; 2] = ["tiktok", "streamlabs"];

#[derive(Debug, Clone, PartialEq)]
pub enum TTSEngines {
    /// Tiktok TTS
    Tiktok,
    /// Streamlabs TTS
    Streamlabs
}
