mod logger;
mod util;

/// Contains all TTS Engines currently supported
pub mod engines;

const DEBUG: bool = false;

#[cfg(test)]
mod tests {
    use crate::engines;
    use anyhow::Result;

    #[test]
    fn tiktok() -> Result<()> {
        engines::tiktok::run_blocking("hello world".to_string(), "tiktok_test.mp3".to_string(), "en_us_006".to_string())?;

        Ok(())
    }

    #[test]
    fn streamlabs_polly() -> Result<()> {
        let voice = engines::streamlabs::Voices::Brian;
        let mut engine = engines::streamlabs::Polly::new(voice, true)?;
        
        let data = engine.gen_tts("hello world, it is me", false)?;
        engine.write_to_file(data, "streamlabs_test.mp3")?;

        Ok(())
    }

    #[test]
    fn streamlabs_polly_timeout_test() -> Result<()> {
        // This is meant to cause a rate limit and wait it
        loop {
            let voice = engines::streamlabs::Voices::Brian;
            let mut engine = engines::streamlabs::Polly::new(voice, false)?;
            
            match engine.gen_tts("hello world, it is me", true) {
                Ok(_) => {},
                Err(e) => {println!("{}",e.to_string()); break },
            }
        }

        Ok(())
    }
}

