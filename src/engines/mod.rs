// For Voice Managment

/// Tikoks TTS engine wrapper
pub mod tiktok;

/// Streamlabs Polly TTS engine wrapper
pub mod streamlabs;

/// A List of TTS Engines currently supported
pub const TTS_ENGINES: [&str; 2] = ["tiktok", "streamlabs"];