// Tiktok Voice

use std::io::{Write};

use anyhow::{bail, Result};

use crate::logger;

pub mod manifest;

// #[allow(unused)]

const BASE_URI: &str = 
"https://api16-normal-useast5.us.tiktokv.com/media/api/text/speech/invoke/?text_speaker=";

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

pub async fn run(text: String, filepath: String, voice: String) -> Result<()> {
    validate_voice(voice.clone())?;

    let client = reqwest::Client::new();

    let endpoint = 
    format!("{0}{1}&req_text={2}&speaker_map_type=0",BASE_URI,voice,text);

    // println!("{}",endpoint);

    //let endpoint = "https://api16-normal-useast5.us.tiktokv.com/media/api/text/speech/invoke/?text_speaker=en_us_006&req_text=hey&speaker_map_type=0";

    let res = client.post(endpoint)
    .header("Content-Length", 0)
    .send().await?;
    

    if res.status() != 200 {
        let err = format!("TTS Server gave code: {}",res.status().clone());
        logger::error(err.clone());
        bail!("{}",err);
    }

    //println!("{}",res.text().await?);
 
    logger::debug("Decoding");
    let res: serde_json::Value = res.json().await?;


    logger::debug("Converting to String");
    let mut data = [res["data"]["v_str"].clone()][0].to_string();

    // Clean the Text - Use nightly?
    data.remove(0);
    data.remove(data.len()-1);

    // println!("{}",data);

    let base64_data = base64::decode(data)?;

    let mut file = if std::path::Path::new(&filepath).exists() {
        std::fs::OpenOptions::new().write(true).open(filepath)?
    } else {
        std::fs::OpenOptions::new().write(true).create_new(true).open(filepath)?
    };

    file.write_all(&base64_data)?;

    //*/

    Ok(())
}


// TODO - Converting to blocking
#[allow(dead_code)]
pub fn run_blocking(text: String, filepath: String, voice: String) -> Result<()> {
    validate_voice(voice.clone())?;

    let client = reqwest::blocking::Client::new();

    let endpoint = 
    format!("{0}{1}&req_text={2}&speaker_map_type=0",BASE_URI,voice,text);

    // println!("{}",endpoint);

    //let endpoint = "https://api16-normal-useast5.us.tiktokv.com/media/api/text/speech/invoke/?text_speaker=en_us_006&req_text=men hey&speaker_map_type=0";

    let res = client.post(endpoint)
    .header("Content-Length", 0)
    .send()?;
    

    if res.status() != 200 {
        let err = format!("TTS Server gave code: {}",res.status().clone());
        logger::error(err.clone());
        bail!("{}",err);
    }

    //println!("{}",res.text().await?);
 
    logger::debug("Decoding");
    let res: serde_json::Value = res.json()?;


    logger::debug("Converting to String");
    let mut data = [res["data"]["v_str"].clone()][0].to_string();

    // Clean the Text - Use nightly?
    data.remove(0);
    data.remove(data.len()-1);

    // println!("{}",data);

    let base64_data = base64::decode(data)?;

    let mut file = if std::path::Path::new(&filepath).exists() {
        std::fs::OpenOptions::new().write(true).open(filepath)?
    } else {
        std::fs::OpenOptions::new().write(true).create_new(true).open(filepath)?
    };

    file.write_all(&base64_data)?;


    Ok(())
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