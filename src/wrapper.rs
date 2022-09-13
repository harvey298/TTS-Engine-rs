use std::str::FromStr;

use anyhow::{Result, bail};

/// TODO - Document

use crate::engines::TTSEngines;

use crate::engines::streamlabs::{Polly, Voices};

#[derive(Debug, Clone)]
pub struct EngineSettings {
    pub engine: TTSEngines,
    pub voice: String,
}

#[derive(Debug, Clone)]
pub struct Engines {
    /// If one engine is unusable use the next on on the list
    priority: Vec<EngineSettings>,

}

impl Engines {

    /// priority is a array of the struct EngineSettings
    pub fn init(priority: Vec<EngineSettings>) -> Self {
        Self{ priority }
    }

    pub async fn run(&self, text: &str, path: &str) -> Result<()> {

        for engine in &self.priority {
            
            if engine.engine == TTSEngines::Streamlabs {
                match polly_wrapper(engine, text, path).await {
                    Ok(_) => { break },
                    Err(_) => {}, // if smth goes wrong don't break the loop
                }
            } else if engine.engine == TTSEngines::Tiktok {
                bail!("Tiktok api down - Use another engine!") // Tiktok doesn't work at the moment
            } else {
                bail!("Failed to create TTS")
            }


        }

        Ok(())
    }
    
}

async fn polly_wrapper(engine: &EngineSettings, text: &str, path: &str) -> Result<()> {
    Polly::new(Voices::from_str(&engine.voice)?, true)?.run(text, path, true).await?;

    Ok(())
}