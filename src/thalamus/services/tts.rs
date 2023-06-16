// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use rouille::Request;
use rouille::Response;


use serde::{Serialize, Deserialize};

use std::thread;
use std::time::Duration;

use std::process::{Command, Stdio};



// http://localhost:8050/api/services/tts?text=hello%20there%20human&primary=larynx:southern_english_female-glow_tts&fallback=opensamfoundation
pub fn handle(request: &Request) -> Result<Response, crate::thalamus::http::Error> {
    if request.url() == "/api/services/tts" {
        let input = request.get_param("text").unwrap();
        let primary = request.get_param("primary").unwrap();
        let fallback = request.get_param("fallback").unwrap();
        return Ok(Response::from_data("audio/wav", crate::thalamus::services::tts::get(input, primary.as_str(), fallback.as_str()).unwrap()));
    }

    if request.url() == "/api/services/tts/voices" {
        return Ok(Response::json(&get_supported_voices()));
    }

    return Ok(Response::empty_404());
}


pub fn get_supported_voices() -> Vec<ThalamusTTSVoice>{
    let mut voices: Vec<ThalamusTTSVoice> = Vec::new();

    voices.push(ThalamusTTSVoice{
        tag: "coqui-tts:en_ljspeech".to_string(),
        gender: "female".to_string(),
        locale: "en-US".to_string(),
        language: "en".to_string(),
        engine: "coqui-tts".to_string(),
        online_api: false
    });

    voices.push(ThalamusTTSVoice{
        tag: "larynx:southern_english_female-glow_tts".to_string(),
        gender: "female".to_string(),
        locale: "en-US".to_string(),
        language: "en".to_string(),
        engine: "larynx".to_string(),
        online_api: false
    });

    voices.push(ThalamusTTSVoice{
        tag: "opensamfoundation".to_string(),
        gender: "female".to_string(),
        locale: "en-US".to_string(),
        language: "en".to_string(),
        engine: "opensamfoundation".to_string(),
        online_api: true
    });

    return voices;

}




pub fn init(args: crate::Args){

    let tts_thead = thread::Builder::new().name("opentts".to_string()).spawn(move || {




        let child = Command::new("/opt/thalamus/bin/docker")
        .arg("run")
        .arg("-it")
        .arg("-p")
        .arg("5500:5500")
        .arg(format!("synesthesiam/opentts:{}", args.lang).as_str())
        .stdout(Stdio::piped())
        .spawn().unwrap();
    
    
        let output = child
        .wait_with_output()
        .expect("failed to wait on child");
    
        log::info!("{}", String::from_utf8_lossy(&output.stdout).to_string());


    });
    match tts_thead{
        Ok(_) => {
            log::info!("opentts server started successfully");
        },
        Err(e) => {
            log::error!("failed to initialize opentts server: {}", e);
        }
    }
}

pub fn get(text: String, primary: &str, fallback: &str) -> Result<Vec<u8>, crate::thalamus::services::Error> {
    let mut primary_had_error = false;

    if primary_had_error{
        log::error!("this should never happen");
    }

    match primary {
        "opensamfoundation" => {
            match fetch_opensam(text.clone()) {
                Ok(x) => {
                    return Ok(x);
                },
                Err(e) => {
                    primary_had_error = true;
                    log::error!("{}", e);
                }
            };

        },
        _ => {
            match fetch_opentts(text.clone(), primary.to_string()) {
                Ok(x) => {
                    return Ok(x);
                },
                Err(e) => {
                    primary_had_error = true;
                    log::error!("{}", e);
                }
            };
        }
    }

    if primary_had_error {
        match fallback {
            "opensamfoundation" => {
                match fetch_opensam(text.clone()) {
                    Ok(x) => {
                        return Ok(x);
                    },
                    Err(e) => {
                        return Err(e);
                    }
                };
    
            },
            _ => {
                match fetch_opentts(text.clone(), fallback.to_string()) {
                    Ok(x) => {
                        return Ok(x);
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }

    // Default
    match fetch_opentts(text.clone(), format!("coqui-tts:en_ljspeech")) {
        Ok(x) => {
            return Ok(x);
        },
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn fetch_opensam(text: String) -> Result<Vec<u8>, crate::thalamus::services::Error> {
    let client = reqwest::blocking::Client::new();
    let bytes = client.get(format!("https://tts.opensam.foundation/api/tts?text={}&speaker_id=&style_wav=", text))
        .basic_auth("sam", Some("87654321"))
        .timeout(Duration::from_secs(5))
        .send()?.bytes()?;
    Ok(bytes.to_vec())
}

pub fn fetch_opentts(text: String, voice: String) -> Result<Vec<u8>, crate::thalamus::services::Error> {
    let client = reqwest::blocking::Client::new();
    let bytes = client.get(format!("http://localhost:5500/api/tts?text={}&voice={}", text, voice))
        .timeout(Duration::from_secs(5))
        .send()?.bytes()?;
    Ok(bytes.to_vec())
}

/// Struct for storing the stats of each node
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThalamusTTSVoice {
    pub tag: String,
    pub gender: String,
    pub language: String,
    pub locale: String,
    pub engine: String,
    pub online_api: bool,
}