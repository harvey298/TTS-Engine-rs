mod logger;
mod util;

/// Contains all TTS Engines currently supported
pub mod engines;

/// Contains Blocking TTS Engines currently supported
#[cfg(feature = "blocking")]
pub mod blocking;

/// A wrapper for all the text to speach engines
#[cfg(feature = "helper")]
pub mod wrapper;

const DEBUG: bool = false;

#[cfg(test)]
mod tests {
    use crate::{engines, blocking};
    use anyhow::Result;

    #[test]
    fn tiktok() -> Result<()> {
        // Put ur session ID where it says Session ID
        blocking::engines::tiktok::TTS::new("en_us_006".to_string(), "".to_string())
            .gen_tts("hello world".to_string(), "tiktok_test.mp3".to_string())?;

        Ok(())
    }

    #[test]
    fn streamlabs_polly() -> Result<()> {
        let voice = blocking::engines::streamlabs::Voices::Brian;
        let mut engine = blocking::engines::streamlabs::Polly::new(voice, true)?;
        
        let data = engine.gen_tts("hello world, it is me", false)?;
        engine.write_to_file(data, "streamlabs_test.mp3")?;

        Ok(())
    }

    #[test]
    fn streamlabs_polly_timeout_test() -> Result<()> {
        // This is meant to cause a rate limit and wait it
        loop {
            let voice = blocking::engines::streamlabs::Voices::Brian;
            let mut engine = blocking::engines::streamlabs::Polly::new(voice, false)?;
            
            match engine.gen_tts("hello world, it is me", true) {
                Ok(_) => {},
                Err(e) => {println!("{}",e.to_string()); break },
            }
        }

        Ok(())
    }
}

