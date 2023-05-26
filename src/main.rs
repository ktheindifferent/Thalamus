// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

// TODO (0.0.1):
// - Dynamically link ffmpeg library for M1/M2 mac (DONE)
// - Move wget functions to rust native library (DONE)
// - Move all cmd functions to rust native library (DONE)
// - Automatic Service Installer for Unix (DONE)
// - whisper vwav generation 
// - llamma.cpp support (DONE)
// - SRGAN (DONE)
// - internal library
// - publish 0.0.1

// TODO (0.0.2):
// - Configurable web pool size
// - Automatic updates via the Open Sam Foundation
// - Nural Style Transfer using ANE
// - Speech Synthesis using ANE
// - Who.io using ANE?
// - Encrypted wav/response support?
// - HTTP/S encryption support?
// - IBM Watson TTS support?
// - deepspeech TTS support
// - Ability to opt-in to send training data to the Open Sam Foundation
// - YoloV8 Support


// Feature List
// - TTS Speech Synthesis using Watson, Deepspeech, apple speech, etc.
// - STT Speech decoding using whisper.cpp with visual wav file generation
// - Llama GPT chat speech generation with multiple model support
// - YoloV8 Image Recognition
// - Image Super Resolution Using SRGAN
// - Web API for easy intergration into existing projects
// - Who.io facial recognition
// - SPREC speech recognition


use rouille::Server;
use rouille::Response;
use simple_logger::SimpleLogger;
use std::path::Path;

extern crate rouille;

pub mod thalamus;

// store application version as a const
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");


fn main() {

    

    // cls
    clearscreen::clear().unwrap();




    // Print Application Art and Version Information
    println!("████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ ");
    println!("   ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      ");
    println!("   ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ ");
    println!("   ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ ");
    println!("   ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████ ");
    println!("Copyright 2021-2023 The Open Sam Foundation (OSF)");
    match VERSION {
        Some(v) => println!("Version: {}", v),
        None => println!("Version: Unknown"),
    };


    if Path::new("/opt/thalamus/").exists() {
        let touch_status = crate::thalamus::tools::touch("/opt/thalamus/output.log".to_string());
        if touch_status.is_ok() {
            SimpleLogger::new().with_colors(true).with_level(log::LevelFilter::Warn).with_timestamps(true).with_output_file("/opt/thalamus/output.log".to_string()).init().unwrap();
        } else {
            SimpleLogger::new().with_colors(true).with_level(log::LevelFilter::Warn).with_timestamps(true).init().unwrap();
        }
    } else {
        simple_logger::SimpleLogger::new().with_colors(true).with_level(log::LevelFilter::Warn).with_timestamps(true).init().unwrap();
    }


    
    sudo::with_env(&["LIBTORCH", "LD_LIBRARY_PATH", "PG_DBNAME", "PG_USER", "PG_PASS", "PG_ADDRESS"]).unwrap();
    
    match crate::thalamus::setup::install(){
        Ok(_) => log::info!("Installed thalamus"),
        Err(e) => log::error!("Error installing thalamus: {}", e),
    };


    let server = Server::new("0.0.0.0:8050", |request| {
        match crate::thalamus::http::handle(request){
            Ok(request) => {
                log::info!("{:?}", request);
                return request;
            },
            Err(err) => {
                log::error!("HTTP_ERROR: {}", err);
                return Response::empty_404();
            }
        }
    }).unwrap().pool_size(6);

    loop {
        server.poll();
    }
}


