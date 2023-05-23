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
use std::thread;

use std::time::Duration;
use std::io::Write;




// /opt/thalamus/bin/whisper -m /opt/thalamus/models/ggml-* -f ./output.wav -otxt
pub fn whisper(file_path: String, method: &str) -> Result<String, crate::thalamus::services::Error> {

    // Force all input to become wav@16khz
    match crate::thalamus::tools::cmd(format!("ffmpeg -i {} -ar 16000 -ac 1 -c:a pcm_s16le {}.16.wav", file_path, file_path)){
        Ok(_) => (),
        Err(e) => return Err(crate::thalamus::services::Error::from(e))
    };

    // Execute Whisper
    match method {
        "tiny" => println!("{}", crate::thalamus::tools::cmd(format!("/opt/thalamus/bin/whisper -m /opt/thalamus/models/ggml-tiny.bin -f {}.16.wav -otxt", file_path))?),
        "base" => println!("{}", crate::thalamus::tools::cmd(format!("/opt/thalamus/bin/whisper -m /opt/thalamus/models/ggml-base.bin -f {}.16.wav -otxt", file_path))?),
        "medium" => println!("{}", crate::thalamus::tools::cmd(format!("/opt/thalamus/bin/whisper -m /opt/thalamus/models/ggml-medium.bin -f {}.16.wav -otxt", file_path))?),
        "large" => println!("{}", crate::thalamus::tools::cmd(format!("/opt/thalamus/bin/whisper -m /opt/thalamus/models/ggml-large.bin -f {}.16.wav -otxt", file_path))?),
        &_ => println!("{}", crate::thalamus::tools::cmd(format!("/opt/thalamus/bin/whisper -m /opt/thalamus/models/ggml-tiny.bin -f {}.16.wav -otxt", file_path))?)
    };
    
    // Copy the results to memory
    let data = std::fs::read_to_string(format!("{}.16.wav.txt", file_path).as_str()).unwrap();

    // Cleanup
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(60000));
        crate::thalamus::tools::cmd(format!("rm {}*", file_path)).unwrap();
    });

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

        log::info!("Unpacking coreml.sh...");
        let data = include_bytes!("../../../packages/whisper/coreml.sh");
        let mut pos = 0;
        let mut buffer = File::create("/opt/thalamus/models/coreml.sh")?;
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

        // Fix permissions
        match crate::thalamus::tools::cmd(format!("chmod -R 777 /opt/thalamus/models")){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod /opt/thalamus")),
        }

        // Configure Miniconda and Generate ML models
        match crate::thalamus::tools::decmd(format!("/opt/thalamus/models/coreml.sh")){
            Ok(x) => {
                println!("{:?}", x);
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to generate coreml models")),
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



    
    return Ok(Response::empty_404());
}