// Tiktok Voice
// Not maintained anymore

use std::{io::{Write}, sync::Arc};

use anyhow::{bail, Result};
use reqwest::{header::{SET_COOKIE, self}, Url, cookie::Jar};

use crate::{logger, util::write_data};

pub mod manifest;

// #[allow(unused)]
const BASE: &str = "https://tiktokv.com";
const BASE_URI: &str = 
"https://api22-normal-c-useast1a.tiktokv.com/media/api/text/speech/invoke/?text_speaker=";

const USER_AGENT: &str =
"com.zhiliaoapp.musically/2022600030 (Linux; U; Android 7.1.2; es_ES; SM-G988N; Build/NRD90M;tt-ok/3.12.13.1)";

const NON_HUMAN_VOICES: [&str; 6] = [  // DISNEY VOICES
"en_us_ghostface",  // Ghost Face
"en_us_chewbacca",  // Chewbacca
"en_us_c3po",  // C3PO
"en_us_stitch",  // Stitch
"en_us_stormtrooper",  // Stormtrooper
"en_us_rocket",  // Rocket
];

const ENGLISH_HUMAN_VOICES: [&str; 10] = [
    "en_au_001",  // English AU - Female
    "en_au_002",  // English AU - Male
    "en_uk_001",  // English UK - Male 1
    "en_uk_003",  // English UK - Male 2
    "en_us_001",  // English US - Female (Int. 1)
    "en_us_002",  // English US - Female (Int. 2)
    "en_us_006",  // English US - Male 1
    "en_us_007",  // English US - Male 2
    "en_us_009",  // English US - Male 3
    "en_us_010",
];

const NON_ENGLISH_VOICES: [&str; 18] = [
    "fr_001",  // French - Male 1
    "fr_002",  // French - Male 2
    "de_001",  // German - Female
    "de_002",  // German - Male
    "es_002",  // Spanish - Male
    // AMERICA VOICES
    "es_mx_002",  // Spanish MX - Male
    "br_001",  // Portuguese BR - Female 1
    "br_003",  // Portuguese BR - Female 2
    "br_004",  // Portuguese BR - Female 3
    "br_005",  // Portuguese BR - Male
    // ASIA VOICES
    "id_001",  // Indonesian - Female
    "jp_001",  // Japanese - Female 1
    "jp_003",  // Japanese - Female 2
    "jp_005",  // Japanese - Female 3
    "jp_006",  // Japanese - Male
    "kr_002",  // Korean - Male 1
    "kr_003",  // Korean - Female
    "kr_004",  // Korean - Male 2
];

#[derive(Debug, Clone)]
pub struct TTS {
    voice: String, // Temp
    session_id: String
}

impl TTS {
    pub fn new(voice: String, session_id: String) -> Self {

        Self { voice, session_id }
    }

    pub fn gen_tts(&self, text: String, filepath: String) -> Result<()> {
        validate_voice(self.voice.clone())?;
    
        let endpoint = 
        format!("{0}{1}&req_text={2}&speaker_map_type=0&aid=1233",BASE_URI,self.voice,text);
    
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Length", header::HeaderValue::from_static("0"));
        headers.insert("Cookie", header::HeaderValue::from_str(&format!("sessionid={}",self.session_id))?);
    
        let client = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .cookie_store(true)
            .default_headers(headers)
            .build()?;
    
        let res = client.post(endpoint)
        .header("Content-Length", 0)
        .send()?;
        
    
        if res.status() != 200 {
            let err = format!("TTS Server gave code: {}",res.status().clone());
            logger::error(err.clone());
            bail!("{}",err);
        }
     
        logger::debug("Decoding");
        let res: serde_json::Value = res.json()?;
    
        logger::debug("Converting to String");
        let mut data = [res["data"]["v_str"].clone()][0].to_string();
    
        // Clean the Text - Use nightly?
        data.remove(0);
        data.remove(data.len()-1);
    
        let base64_data = base64::decode(data)?;
    
        write_data(&filepath, base64_data)?;
    
        Ok(())
    }
}

fn validate_voice(wanted_voice: String) -> Result<()> {
    //drop(wanted_voice);

    //if !NON_HUMAN_VOICES.contains(&&wanted_voice[..]) || !ENGLISH_HUMAN_VOICES.contains(&&wanted_voice[..]) || !NON_ENGLISH_VOICES.contains(&&wanted_voice[..]) {
    //    bail!("Voice {} Doesn't Exist!", wanted_voice);
    //}

    let mut all_voices = Vec::new();

    for voice in NON_HUMAN_VOICES { all_voices.push(voice); }

    for voice in ENGLISH_HUMAN_VOICES { all_voices.push(voice); }
    
    for voice in NON_ENGLISH_VOICES { all_voices.push(voice); }
    
    if !all_voices.contains(&&wanted_voice[..]) { bail!("Voice {} Doesn't Exist!", wanted_voice); }

    Ok(())
}