use std::io::Write;

use anyhow::{Result, bail};

use super::{ENGLISH_HUMAN_VOICES, NON_HUMAN_VOICES, NON_ENGLISH_VOICES};

pub const MANIFEST_FILE: &str = "TikTok_Voices.md";

#[derive(Debug, Clone)]
pub struct TiktokVoice {
    pub voice: String,
    pub desc: String
}

pub fn get_all_voices(class: String) -> Result<Vec<TiktokVoice>>  {
    let mut voices = Vec::new();

    let class = class.to_lowercase();

    if class == "english" {
        let mut itr = 0;

        for v in ENGLISH_HUMAN_VOICES {
            voices.push(TiktokVoice{ voice: v.to_string(), desc: ENGLISH_HUMAN_VOICES_DESC[itr].to_string()  });
            itr += 1;
        }

    } else if class == "non english" {
        let mut itr = 0;

        for v in NON_ENGLISH_VOICES {
            voices.push(TiktokVoice{ voice: v.to_string(), desc: NON_ENGLISH_VOICES_DESC[itr].to_string()  });
            itr += 1;
        }

    } else if class == "non human" {

        let mut itr = 0;

        for v in NON_HUMAN_VOICES {
            voices.push(TiktokVoice{ voice: v.to_string(), desc: NON_HUMAN_VOICES_DESC[itr].to_string()  });
            itr += 1;
        }

    } else if class == "all" {

        for v in get_all_voices("english".to_owned())? {
            voices.push(v)
        }

        for v in get_all_voices("non english".to_owned())? {
            voices.push(v)
        }

        for v in get_all_voices("non human".to_owned())? {
            voices.push(v)
        }

    } else { bail!("{} is not a Valid Voice Class!", class) }

    Ok(voices)
}

#[allow(unused)]
const ENGLISH_HUMAN_VOICES_DESC: [&str; 10] = [
    "English AU - Female",  // English AU - Female
    "English AU - Male",  // English AU - Male
    "English UK - Male 1",  // English UK - Male 1
    "English UK - Male 2",  // English UK - Male 2
    "English US - Female (Int. 1)",  // English US - Female (Int. 1)
    "English US - Female (Int. 2)",  // English US - Female (Int. 2)
    "English US - Male 1",  // English US - Male 1
    "English US - Male 2",  // English US - Male 2
    "English US - Male 3",  // English US - Male 3
    "English US", 
];

#[allow(unused)]
const NON_ENGLISH_VOICES_DESC: [&str; 18] = [
    "French - Male 1",
    "French - Male 2",
    "German - Female",
    "German - Male",
    "Spanish - Male",
    "Spanish MX - Male",
    "Portuguese BR - Female 1",
    "Portuguese BR - Female 2",
    "Portuguese BR - Female 3",
    "Portuguese BR - Male",
    "Indonesian - Female",
    "Japanese - Female 1",
    "Japanese - Female 2",
    "Japanese - Female 3",
    "Japanese - Male",
    "Korean - Male 1",
    "Korean - Female",
    "Korean - Male 2",
];

#[allow(unused)]
const NON_HUMAN_VOICES_DESC: [&str; 6] = [  // DISNEY VOICES
"Ghost Face",  // Ghost Face
"Chewbacca",  // Chewbacca
"C3PO",  // C3PO
"Stitch",  // Stitch
"Stormtrooper",  // Stormtrooper
"Rocket",  // Rocket
];

#[allow(unused_mut)]
pub fn write_voices_manifest() -> Result<()> {

    let eng_voices = get_all_voices("english".to_string())?;
    let mut eng_voice_str = String::new();
    for v in eng_voices {
        eng_voice_str += format!("{} - {}\n",v.voice,v.desc).as_str();
    }

    let eng_voices = get_all_voices("non human".to_string())?;
    let mut non_human_voice_str = String::new();
    for v in eng_voices {
        non_human_voice_str += format!("{} - {}\n",v.voice,v.desc).as_str();
    }

    let eng_voices = get_all_voices("non english".to_string())?;
    let mut non_eng_voice_str = String::new();
    for v in eng_voices {
        non_eng_voice_str += format!("{} - {}\n",v.voice,v.desc).as_str();
    }

    let output = format!(
r#"# TikTok TTS Engine Voices

### ENGLISH VOICES ###
{}
### NON ENGLISH VOICES ###
{}
### NON HUMAN VOICES ###
{}
"#, eng_voice_str, non_eng_voice_str, non_human_voice_str
);


    let mut file = if std::path::Path::new(MANIFEST_FILE).exists() {
        std::fs::OpenOptions::new().write(true).open(MANIFEST_FILE)?
    } else { std::fs::OpenOptions::new().write(true).create_new(true).open(MANIFEST_FILE)? };

    file.write_all(output.as_bytes())?;

    // write_data_to_file(output, MANIFEST_FILE, None)?;

    Ok(())
}