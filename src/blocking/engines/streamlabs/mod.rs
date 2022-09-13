use std::{str::FromStr, fmt, collections::HashMap, time::{Instant, UNIX_EPOCH, SystemTime, Duration}, thread::sleep};

/// Streamlabs Polly Module

use anyhow::{Result, bail};
use bytes::{Bytes, Buf};
use serde::{Serialize, Deserialize};

use crate::util::write_bytes;

/// A text list of all the voices
pub const VOICES: [&str; 15] = [
    "Brian",
    "Emma",
    "Russell",
    "Joey",
    "Matthew",
    "Joanna",
    "Kimberly",
    "Amy",
    "Geraint",
    "Nicole",
    "Justin",
    "Ivy",
    "Kendra",
    "Salli",
    "Raveena"
];

const POLLY_URL: &str = "https://streamlabs.com/polly/speak";

/// For every voice Streamlbs polly supports
#[derive(Debug, Clone)]
pub enum Voices {
    /// English - Male - British
    Brian, 

    /// Englisy - Female - British
    Emma, 

    /// English - Male - Australian
    Russell, 

    /// English - Male - American
    Joey, 

    /// English - Male - American
    Matthew, 

    /// English - Female - American
    Joanna, 

    /// English - Female - American
    Kimberly, 

    /// English - Female - British
    Amy, 

    /// English - Male - Welsh
    Geraint,

    /// English - Female - Australian
    Nicole, 

    /// English - Male - American
    Justin, 

    /// English - Female - American
    Ivy, 

    /// English - Female - American
    Kendra, 

    /// English - Female - American
    Salli, 

    /// English - Female - Indian
    Raveena, 
}

impl fmt::Display for Voices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Voices {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {

        let s = s.to_lowercase();
        // sorry for spaget
        if s == VOICES[0].to_lowercase() { Ok(Self::Brian)

        } else if s == VOICES[1].to_lowercase() { Ok(Self::Emma)

        } else if s == VOICES[2].to_lowercase() { Ok(Self::Russell)

        } else if s == VOICES[3].to_lowercase() { Ok(Self::Joey)

        } else if s == VOICES[4].to_lowercase() { Ok(Self::Matthew)

        } else if s == VOICES[5].to_lowercase() { Ok(Self::Joanna)

        } else if s == VOICES[6].to_lowercase() { Ok(Self::Kimberly)

        } else if s == VOICES[7].to_lowercase() { Ok(Self::Amy)

        } else if s == VOICES[8].to_lowercase() { Ok(Self::Geraint)

        } else if s == VOICES[9].to_lowercase() { Ok(Self::Nicole)

        } else if s == VOICES[10].to_lowercase() { Ok(Self::Justin)

        } else if s == VOICES[11].to_lowercase() { Ok(Self::Ivy)

        } else if s == VOICES[12].to_lowercase() { Ok(Self::Kendra)

        } else if s == VOICES[13].to_lowercase() { Ok(Self::Salli)

        } else if s == VOICES[14].to_lowercase() { Ok(Self::Raveena)

        } else {
            bail!("Voice Doesn't Exist!")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PollyError {
    pub error: String,
}

/// A Streamlabs Polly TTS wrapper 
/// 
/// To create a new Polly instance is:
/// ``Polly::new(voice, false);``
/// 
/// The `run` function handles generating the audio and saves it to the disk
/// 
/// A voice is the voice Enum called "Voice"
#[derive(Debug, Clone)]
pub struct Polly {
    voice: Voices,
    ror: bool, // Retry On Ratelimit
    ratelimit: Option<RateLimit>
}

#[derive(Debug, Clone)]
struct RateLimit {
    now: Instant,
    duration: u64    
}

impl Polly {
    /// Creates a TTS instance that allows quick access to TTS with a single voice over and over again
    pub fn new(voice: Voices, retry_on_ratelimit: bool) -> Result<Self> {

        Ok(Self{ voice, ratelimit: None, ror: retry_on_ratelimit })
    }

    /// Creates the TTS and saves it to the disk
    pub fn run(&mut self, text: &str, filename: &str, wait_on_ratelimit: bool) -> Result<()> {

        let data = self.gen_tts(text, wait_on_ratelimit)?;
        self.write_to_file(data, filename)?;

        Ok(())
    }

    /// Write the file to the disk
    pub fn write_to_file(&self, data: Bytes, filename: &str) -> Result<()> {

        let data = data.chunk();
        write_bytes(filename, data)?;

        Ok(())
    }

    /// Uses Streamlabs Polly to create the TTS and returns Bytes
    pub fn gen_tts(&mut self, text: &str, wait_on_ratelimit: bool) -> Result<Bytes> {

        if self.ratelimit.is_some() {
            let limit = self.ratelimit.as_ref().unwrap();

            if Instant::now().duration_since(limit.clone().now) < Duration::from_secs(limit.duration) {
                self.ratelimit = None;
            } else {
                if wait_on_ratelimit {
                    sleep(Duration::from_secs(limit.duration));
                } else {
                    bail!("Ratelimit still acitve!")
                }
            }
        }

        let voice = self.voice.to_string();

        #[allow(unused_mut)]
        let mut client = reqwest::blocking::Client::builder();
        
        let client = client.build()?;

        let mut map = HashMap::new();
        map.insert("voice", voice.clone());
        map.insert("text", text.clone().to_string());
        map.insert("service", "polly".to_string());
        
        let res = client.post(POLLY_URL).json(&map).send()?;

        let code = res.status();

        if code != 200 {
            if code == 429 {

                let res_data: u64 = res.headers()["X-RateLimit-Reset"].to_str()?.parse()?;

                let ctime = match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(n) => n.as_secs(),
                    Err(_) => bail!("SystemTime before UNIX EPOCH!"),
                };

                let duration = res_data - ctime;

                self.ratelimit = Some(RateLimit{ now: Instant::now(), duration });

                if wait_on_ratelimit {
                    sleep(Duration::from_secs(duration));
                }
                
                if self.ror {
                   return Ok( self.gen_tts(text, wait_on_ratelimit)? )

                } else {
                    bail!("Ratelimited!")
                }                
            }

            match res.json::<PollyError>() {
                Ok(o) => { bail!("Streamlabs gave: {} | Error is: {}",code, o.error) },
                Err(_) => { bail!("Streamlabs gave: {}",code) },
            }
        }

        let res_data: serde_json::Value = res.json()?;

        if res_data["speak_url"].is_string() {
            let url = res_data["speak_url"].as_str().unwrap();
            let res = client.clone().get(url).send()?.bytes()?;
            Ok(res)

        } else {
            bail!("Failed to use Streamlabs Polly")
        }
    }
}