// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open thalamus Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use rouille::Request;
use rouille::Response;
use rouille::input::post::BufferedFile;
use rouille::post_input;
use serde::{Serialize, Deserialize};

use std::path::Path;
use std::fs::File;

use std::time::{SystemTime, UNIX_EPOCH};

use std::io::Write;




// /opt/thalamus/bin/whisper -m /opt/thalamus/models/ggml-* -f ./output.wav -otxt
pub fn whisper(file_path: String, method: &str) -> Result<String, crate::thalamus::services::Error> {

    // Force all input to become wav@16khz
    match crate::thalamus::tools::wav_to_16000(file_path.clone()){
        Ok(_) => (),
        Err(e) => return Err(crate::thalamus::services::Error::from(e))
    };

    // Execute Whisper
    match method {
        "tiny" => log::warn!("{}", crate::thalamus::tools::whisper("tiny", file_path.as_str())?),
        "base" => log::warn!("{}", crate::thalamus::tools::whisper("base", file_path.as_str())?),
        "medium" => log::warn!("{}", crate::thalamus::tools::whisper("medium", file_path.as_str())?),
        "large" => log::warn!("{}", crate::thalamus::tools::whisper("large", file_path.as_str())?),
        &_ => log::warn!("{}", crate::thalamus::tools::whisper("tiny", file_path.as_str())?)
    };
    
    // Copy the results to memory
    let data = std::fs::read_to_string(format!("{}.16.wav.txt", file_path).as_str())?;

    // Cleanup
    // thread::spawn(move || {
    //     thread::sleep(Duration::from_millis(60000));
    //     crate::thalamus::tools::rm(format!("{}*", file_path).as_str()).unwrap();
    // });

    // Return the results
    return Ok(data);
}


pub fn whisper_vwav(file_path: String, method: &str) -> Result<String, crate::thalamus::services::Error> {

    // Force all input to become wav@16khz
    match crate::thalamus::tools::wav_to_16000(file_path.clone()){
        Ok(_) => (),
        Err(e) => return Err(crate::thalamus::services::Error::from(e))
    };



    // Execute Whisper
    match method {
        "tiny" => log::warn!("{}", crate::thalamus::tools::whisper_owts("tiny", file_path.as_str())?),
        "base" => log::warn!("{}", crate::thalamus::tools::whisper_owts("base", file_path.as_str())?),
        "medium" => log::warn!("{}", crate::thalamus::tools::whisper_owts("medium", file_path.as_str())?),
        "large" => log::warn!("{}", crate::thalamus::tools::whisper_owts("large", file_path.as_str())?),
        &_ => log::warn!("{}", crate::thalamus::tools::whisper_owts("tiny", file_path.as_str())?)
    };
    
    // linux only patch
    #[cfg(all(target_os = "linux"))] {
        crate::thalamus::services::whisper::patch_whisper_wts(format!("{}.16.wav.wts", file_path.clone())).unwrap();
    }
    
    match crate::thalamus::tools::mark_as_executable(format!("{}.16.wav.wts", file_path.clone()).as_str()){
        Ok(_) => {},
        Err(e) => {
            log::error!("{}", e);
        },
    }

    // 
    match crate::thalamus::tools::sh(format!("{}.16.wav.wts", file_path.clone()).as_str()){
        Ok(_) => {},
        Err(e) => {
            log::error!("{}", e);
        },
    }


  
    // Return the results
    return Ok(format!("{}.16.wav.mp4", file_path.clone()));
}

// Patch linux whisper WTS files
pub fn patch_whisper_wts(file_path: String) -> Result<(), crate::thalamus::services::Error>{
    let mut data = std::fs::read_to_string(format!("{}", file_path).as_str())?;
    data = data.replace("ffmpeg", "/opt/thalamus/bin/ffmpeg").replace("/System/Library/Fonts/Supplemental/Courier New Bold.ttf","/opt/thalamus/fonts/courier.ttf");
    std::fs::remove_file(format!("{}", file_path).as_str())?;
    std::fs::write(file_path, data)?;
    return Ok(());
}


// TODO: Compile whisper for raspi and patch installer
pub fn install() -> Result<(), crate::thalamus::setup::Error> {

    
    if !Path::new("/opt/thalamus/models/ggml-tiny.bin").exists(){
        log::warn!("ggml-tiny.bin is missing.....downloading it from https://huggingface.co/");
        match crate::thalamus::tools::download("/opt/thalamus/models/ggml-tiny.bin", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin"){
            Ok(_) => {
                log::info!("Stored model ggml-tiny.bin in /opt/thalamus/models/");
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download tiny whisper model").into())
        }
    } else {
        if 77691713 != crate::thalamus::tools::get_file_size("/opt/thalamus/models/ggml-tiny.bin")?{
            // hash check
            // be07e048e1e599ad46341c8d2a135645097a538221678b7acdd1b1919c6e1b21
            if crate::thalamus::tools::hash_check("/opt/thalamus/models/ggml-tiny.bin").unwrap() != "be07e048e1e599ad46341c8d2a135645097a538221678b7acdd1b1919c6e1b21".to_string(){
                log::warn!("ggml-tiny.bin failed the hash check.....re-downloading it from https://huggingface.co/");
                match crate::thalamus::tools::download("/opt/thalamus/models/ggml-tiny.bin", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin"){
                    Ok(_) => {
                        log::info!("Stored model ggml-tiny.bin in /opt/thalamus/models/");
                    },
                    Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download tiny whisper model").into())
                }
            }
        }

    }

    if !Path::new("/opt/thalamus/models/ggml-base.bin").exists(){
        log::warn!("ggml-base.bin is missing.....downloading it from https://huggingface.co/");
        match crate::thalamus::tools::download("/opt/thalamus/models/ggml-base.bin", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin"){
            Ok(_) => {
                log::info!("Stored model ggml-base.bin in /opt/thalamus/models/");
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download base whisper model").into())
        }
    } else {
        if 147951465 != crate::thalamus::tools::get_file_size("/opt/thalamus/models/ggml-base.bin")?{
            // hash check
            // 60ed5bc3dd14eea856493d334349b405782ddcaf0028d4b5df4088345fba2efe
            if crate::thalamus::tools::hash_check("/opt/thalamus/models/ggml-base.bin").unwrap() != "60ed5bc3dd14eea856493d334349b405782ddcaf0028d4b5df4088345fba2efe".to_string(){
                log::warn!("ggml-base.bin failed the hash check.....re-downloading it from https://huggingface.co/");
                match crate::thalamus::tools::download("/opt/thalamus/models/ggml-base.bin", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin"){
                    Ok(_) => {
                        log::info!("Stored model ggml-base.bin in /opt/thalamus/models/");
                    },
                    Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download base whisper model").into())
                }
            }
        }
    }

    if !Path::new("/opt/thalamus/models/ggml-medium.bin").exists(){
        log::warn!("ggml-medium.bin is missing.....downloading it from https://huggingface.co/");
        match crate::thalamus::tools::download("/opt/thalamus/models/ggml-medium.bin", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin"){
            Ok(_) => {
                log::info!("Stored model ggml-medium.bin in /opt/thalamus/models/");
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download medium whisper model").into())
        }
    } else {
        if 1533763059 != crate::thalamus::tools::get_file_size("/opt/thalamus/models/ggml-medium.bin")?{
            // hash check
            // 6c14d5adee5f86394037b4e4e8b59f1673b6cee10e3cf0b11bbdbee79c156208
            if crate::thalamus::tools::hash_check("/opt/thalamus/models/ggml-medium.bin").unwrap() != "6c14d5adee5f86394037b4e4e8b59f1673b6cee10e3cf0b11bbdbee79c156208".to_string(){
                log::warn!("ggml-medium.bin failed the hash check.....re-downloading it from https://huggingface.co/");
                match crate::thalamus::tools::download("/opt/thalamus/models/ggml-medium.bin", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin"){
                    Ok(_) => {
                        log::info!("Stored model ggml-medium.bin in /opt/thalamus/models/");
                    },
                    Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download medium whisper model").into())
                }
            }
        }
    }

    if !Path::new("/opt/thalamus/models/ggml-large.bin").exists(){
        log::warn!("ggml-large.bin is missing.....downloading it from https://huggingface.co/");
        match crate::thalamus::tools::download("/opt/thalamus/models/ggml-large.bin", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large.bin"){
            Ok(_) => {
                log::info!("Stored model ggml-large.bin in /opt/thalamus/models/");
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download large whisper model").into())
        }
    } else {
        if 3094623691 != crate::thalamus::tools::get_file_size("/opt/thalamus/models/ggml-large.bin")?{
            // hash check
            // 9a423fe4d40c82774b6af34115b8b935f34152246eb19e80e376071d3f999487
            if crate::thalamus::tools::hash_check("/opt/thalamus/models/ggml-large.bin").unwrap() != "9a423fe4d40c82774b6af34115b8b935f34152246eb19e80e376071d3f999487".to_string(){
                log::warn!("ggml-large.bin failed the hash check.....re-downloading it from https://huggingface.co/");
                match crate::thalamus::tools::download("/opt/thalamus/models/ggml-large.bin", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large.bin"){
                    Ok(_) => {
                        log::info!("Stored model ggml-large.bin in /opt/thalamus/models/");
                    },
                    Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download medium whisper model").into())
                }
            }
        }
    }




    

    // crate::thalamus::tools::extract_zip("/opt/thalamus/models/models.zip", "/opt/thalamus/models/")?;

    // match crate::thalamus::tools::rmd("/opt/thalamus/models/models.zip"){
    //     Ok(_) => (),
    //     Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to remove models.zip"))
    // }

    #[cfg(target_arch = "x86_64")]{
        if !Path::new("/opt/thalamus/bin/whisper").exists(){
            log::info!("Installing whisper (x86_64) /opt/thalamus/bin/whisper");
            crate::thalamus::tools::download("/opt/thalamus/bin/whisper", "https://www.dropbox.com/s/ovcjbhmdysnlyyn/main")?;
        }

        if !Path::new("/opt/thalamus/bin/ffmpeg").exists(){
            log::info!("Installing ffmpeg (x86_64) /opt/thalamus/bin/ffmpeg");
            crate::thalamus::tools::download("/opt/thalamus/bin/ffmpeg", "https://www.dropbox.com/s/j91btel44c37g98/ffmpeg")?;
        }
        match crate::thalamus::tools::mark_as_executable("/opt/thalamus/bin/ffmpeg"){
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod ffmpeg").into())
        }

    }

    // Apple M1/M2
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {

        if !Path::new("/opt/thalamus/bin/whisper").exists(){
            log::info!("Installing whisper (aarch64) /opt/thalamus/bin");
            crate::thalamus::tools::download("/opt/thalamus/bin/whisper", "https://www.dropbox.com/s/1fl35hlp5op2pfn/main")?;
        }

        if !Path::new("/opt/thalamus/models/convert-whisper-to-coreml.py").exists(){
            log::info!("Unpacking convert-whisper-to-coreml.py...");
            crate::thalamus::tools::download("/opt/thalamus/models/convert-whisper-to-coreml.py", "https://www.dropbox.com/s/hu40n989phv0igk/convert-whisper-to-coreml.py")?;
        }

        if !Path::new("/opt/thalamus/models/generate-coreml-model.sh").exists(){
            log::info!("Unpacking generate-coreml-model.sh...");
            crate::thalamus::tools::download("/opt/thalamus/models/generate-coreml-model.sh", "https://www.dropbox.com/s/8h59bw07q8tbaak/generate-coreml-model.sh")?;
        }

        if !Path::new("/opt/thalamus/models/coreml.sh").exists(){
            log::info!("Unpacking coreml.sh...");
            crate::thalamus::tools::download("/opt/thalamus/models/coreml.sh", "https://www.dropbox.com/s/ico9dlti77v6k6u/coreml.sh")?;
        }

        // Fix permissions
        match crate::thalamus::tools::fix_permissions("/opt/thalamus/models"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod /opt/thalamus").into()),
        }
        match crate::thalamus::tools::mark_as_executable("/opt/thalamus/models/coreml.sh"){
            Ok(_) => {},
            Err(_) => {},
        }

        match crate::thalamus::tools::mark_as_executable("/opt/thalamus/models/generate-coreml-model.sh"){
            Ok(_) => {},
            Err(_) => {},
        }

        // Configure Miniconda and Generate ML models if necessary
        if !Path::new("/opt/thalamus/models/coreml-encoder-tiny.mlpackage").exists() || !Path::new("/opt/thalamus/models/coreml-encoder-large.mlpackage").exists(){
            log::warn!("CoreML Encoders are missing...please be patient while they are being generated. This may take a while. Future launches will be faster.");
            match crate::thalamus::tools::sh("/opt/thalamus/models/coreml.sh"){
                Ok(_) => {},
                Err(_) => {},
            }  
            log::warn!("CoreML encoders have been generated. Please check the log for more information.");  
        }
    }

    if !Path::new("/opt/thalamus/fonts/courier.ttf").exists(){
        crate::thalamus::tools::download("/opt/thalamus/fonts/courier.ttf", "https://www.dropbox.com/s/qip7w9ik3a15qso/courier.ttf")?;
    }

    match crate::thalamus::tools::mark_as_executable("/opt/thalamus/bin/whisper"){
        Ok(_) => (),
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod whisper").into())
    }

    Ok(())
}




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct STTReply {
    pub text: String,
    pub time: f64,
    pub response_type: Option<String>,
}



pub fn handle(request: &Request) -> Result<Response, crate::thalamus::http::Error> {
    
   

    
    if request.url() == "/api/services/whisper" {

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

        let input = post_input!(request, {
            speech: BufferedFile,
            method: String
        })?;

        let tmp_file_path = format!("/opt/thalamus/tmp/{}.wav", timestamp.clone());
        let mut file = File::create(tmp_file_path.clone())?;
        file.write_all(&input.speech.data)?;

        let stt = whisper(tmp_file_path, input.method.as_str())?;

        let reply = STTReply{
            text: stt,
            time: timestamp as f64,
            response_type: None
        };

        log::info!("{}", reply.text.clone());

        return Ok(Response::json(&reply));
      
    }


    if request.url() == "/api/services/whisper/vwav"{

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

        let input = post_input!(request, {
            speech: BufferedFile,
            method: String
        })?;

        let tmp_file_path = format!("/opt/thalamus/tmp/{}.wav", timestamp.clone());
        let mut file = File::create(tmp_file_path.clone())?;
        file.write_all(&input.speech.data)?;

        let output_path = whisper_vwav(tmp_file_path, input.method.as_str())?;

        let outfile = File::open(output_path.as_str()).unwrap();

        let response = Response::from_file("video/mp4", outfile);
        return Ok(response);
       
                
    }

    
    return Ok(Response::empty_404());
}