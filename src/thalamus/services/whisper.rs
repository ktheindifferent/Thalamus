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
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs::File;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::thread;
use std::thread::Builder;
use std::io::Write;




// /opt/thalamus/bin/whisper -m /opt/thalamus/models/ggml-* -f ./output.wav -otxt
pub fn whisper(file_path: String, method: &str) -> Result<String, crate::thalamus::services::Error> {

    // Force all input to become wav@16khz
    crate::thalamus::tools::cmd(format!("ffmpeg -i {} -ar 16000 -ac 1 -c:a pcm_s16le {}.16.wav", file_path, file_path));

    // Execute Whisper
    match method {
        "quick" => crate::thalamus::tools::cmd(format!("/opt/thalamus/bin/whisper -m /opt/thalamus/models/ggml-tiny.bin -f {}.16.wav -otxt -t 4", file_path)),
        "large" => crate::thalamus::tools::cmd(format!("/opt/thalamus/bin/whisper -m /opt/thalamus/models/ggml-large.bin -f {}.16.wav -otxt -t 4", file_path)),
        &_ => crate::thalamus::tools::cmd(format!("/opt/thalamus/bin/whisper -m /opt/thalamus/models/ggml-tiny.bin -f {}.16.wav -otxt -t 4", file_path))
    };
    
    // Copy the results to memory
    let data = std::fs::read_to_string(format!("{}.16.wav.txt", file_path).as_str())?;

    // Cleanup
    std::fs::remove_file(format!("{}.16.wav", file_path).as_str())?;
    std::fs::remove_file(format!("{}.16.wav.txt", file_path).as_str())?;

    // Return the results
    return Ok(data);
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
pub fn install() -> std::io::Result<()> {

    if !Path::new("/opt/thalamus/models/ggml-tiny.bin").exists(){
        log::warn!("ggml-tiny.bin is missing.....downloading it from https://huggingface.co/");
        match crate::thalamus::tools::cmd(format!("wget -O /opt/thalamus/models/ggml-tiny.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin")){
            Ok(_) => {
                log::info!("Stored model ggml-tiny.bin in /opt/thalamus/models/");
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download tiny whisper model"))
        }
    }

    if !Path::new("/opt/thalamus/models/ggml-base.bin").exists(){
        log::warn!("ggml-base.bin is missing.....downloading it from https://huggingface.co/");
        match crate::thalamus::tools::cmd(format!("wget -O /opt/thalamus/models/ggml-base.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin")){
            Ok(_) => {
                log::info!("Stored model ggml-base.bin in /opt/thalamus/models/");
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download base whisper model"))
        }
    }

    if !Path::new("/opt/thalamus/models/ggml-medium.bin").exists(){
        log::warn!("ggml-medium.bin is missing.....downloading it from https://huggingface.co/");
        match crate::thalamus::tools::cmd(format!("wget -O /opt/thalamus/models/ggml-medium.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin")){
            Ok(_) => {
                log::info!("Stored model ggml-medium.bin in /opt/thalamus/models/");
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download medium whisper model"))
        }
    }

    if !Path::new("/opt/thalamus/models/ggml-large.bin").exists(){
        log::warn!("ggml-large.bin is missing.....downloading it from https://huggingface.co/");
        match crate::thalamus::tools::cmd(format!("wget -O /opt/thalamus/models/ggml-large.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large.bin")){
            Ok(_) => {
                log::info!("Stored model ggml-large.bin in /opt/thalamus/models/");
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download large whisper model"))
        }
    }

    #[cfg(target_arch = "x86_64")]{
        log::info!("Installing whisper (x86_64) /opt/thalamus/bin/whisper");
        let data = include_bytes!("../../../packages/whisper/main-amd64");
        let mut pos = 0;
        let mut buffer = File::create("/opt/thalamus/bin/whisper")?;
        while pos < data.len() {
            let bytes_written = buffer.write(&data[pos..])?;
            pos += bytes_written;
        }

        log::info!("Unpacking models.zip...");
        let data = include_bytes!("../../../packages/whisper/models.zip");
        let mut pos = 0;
        let mut buffer = File::create("/opt/thalamus/models/models.zip")?;
        while pos < data.len() {
            let bytes_written = buffer.write(&data[pos..])?;
            pos += bytes_written;
        }
    
        crate::thalamus::tools::extract_zip("/opt/thalamus/models/models.zip", format!("/opt/thalamus/models/"));
        match crate::thalamus::tools::cmd(format!("rm -rf /opt/thalamus/models/models.zip")){
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to remove models.zip"))
        }
        match crate::thalamus::tools::cmd(format!("mv /opt/thalamus/models/models/* /opt/thalamus/models/")){
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to move model data"))
        }
        match crate::thalamus::tools::cmd(format!("rm -rf /opt/thalamus/models/models/")){
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to cleanup models data"))
        }
    }

    // Apple M1/M2
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
        log::info!("Installing whisper (aarch64) /opt/thalamus/bin");
        let data = include_bytes!("../../../packages/whisper/apple/main");
        let mut pos = 0;
        let mut buffer = File::create("/opt/thalamus/bin/whisper")?;
        while pos < data.len() {
            let bytes_written = buffer.write(&data[pos..])?;
            pos += bytes_written;
        }

        log::info!("Unpacking models.zip...");
        let data = include_bytes!("../../../packages/whisper/models.zip");
        let mut pos = 0;
        let mut buffer = File::create("/opt/thalamus/models/models.zip")?;
        while pos < data.len() {
            let bytes_written = buffer.write(&data[pos..])?;
            pos += bytes_written;
        }
    
        crate::thalamus::tools::extract_zip("/opt/thalamus/models/models.zip", format!("/opt/thalamus/models/"));
        match crate::thalamus::tools::cmd(format!("rm -rf /opt/thalamus/models/models.zip")){
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to remove models.zip"))
        }
        match crate::thalamus::tools::cmd(format!("mv /opt/thalamus/models/models/* /opt/thalamus/models/")){
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to move model data"))
        }
        match crate::thalamus::tools::cmd(format!("rm -rf /opt/thalamus/models/models/")){
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to cleanup models data"))
        }
    }

    let data = include_bytes!("../../../fonts/courier.ttf");
    let mut pos = 0;
    let mut buffer = File::create("/opt/thalamus/fonts/courier.ttf")?;
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    let data = include_bytes!("../../../packages/ffmpeg/amd64/ffmpeg");
    let mut pos = 0;
    let mut buffer = File::create("/opt/thalamus/bin/ffmpeg")?;
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }

    match crate::thalamus::tools::cmd(format!("chmod +x /opt/thalamus/bin/ffmpeg")){
        Ok(_) => (),
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod ffmpeg"))
    }
    
    match crate::thalamus::tools::cmd(format!("chmod +x /opt/thalamus/bin/whisper")){
        Ok(_) => (),
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod whisper"))
    }

    Ok(())
}








pub fn handle(request: &Request) -> Result<Response, crate::thalamus::http::Error> {
    
   

    
    if request.url() == "/api/services/stt" {


        let data = post_input!(request, {
            audio_data: rouille::input::post::BufferedFile,
        })?;


        let tmp_file_path = format!("/opt/thalamus/tmp/{}.wav", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);

        let mut file = File::create(tmp_file_path.clone()).unwrap();
        file.write_all(&data.audio_data.data).unwrap();


        let mut idk = upload(tmp_file_path).unwrap();

        // TODO - Spawn thread to store audio/text files as an observation.
        // TODO - Spawn sprec thread to identify speaker.
        // TODO - Spawn thread to process thalamus brain.py. (maybe, might execute in js runtime instead)
        
        
        // If idk.text contains "thalamus" then redirect request to io api
        if idk.text.contains("thalamus") {
            return Ok(Response::redirect_303(format!("/api/io?input={}", idk.text.replace("thalamus ", ""))));
        }

        idk.response_type = Some(format!("stt"));

        return Ok(Response::json(&idk));
    }



    
    return Ok(Response::empty_404());
}

pub fn init(){

    let stt_thread = thread::Builder::new().name("stt".to_string()).spawn(move || {
        crate::thalamus::tools::cmd(format!("docker run -p 8002:8000 p0indexter/stt"));
    });
    match stt_thread{
        Ok(_) => {
            log::info!("stt server started successfully");
        },
        Err(e) => {
            log::error!("failed to initialize stt server: {}", e);
        }
    }
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct STTReply {
    pub text: String,
    pub time: f64,
    pub response_type: Option<String>,
}


// TODO - Use foundation stt public server first, fallback to local server after 2 seconds for offline avaliablity
// TODO - Find another method besided multipart....too many dependencies
pub fn upload(tmp_file_path: String) -> Result<STTReply, crate::thalamus::services::Error> {

    // return Ok(STTReply{
    //     text: String::new(),
    //     time: 0.0,
    //     response_type: None,
    // });

    let form = reqwest::blocking::multipart::Form::new().file("speech", tmp_file_path.as_str())?;


    let client = reqwest::blocking::Client::new();

    Ok(client.post(format!("https://stt.openthalamus.foundation/api/v1/stt"))
    .multipart(form)
    .send()?.json()?)

    // Ok(client.post(format!("http://localhost:8002/api/v1/stt"))
    //     .multipart(form)
    //     .send()?.json()?)
}