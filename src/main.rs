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
// - whisper vwav generation (DONE)
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

use std::error::Error;
use tokio::task;
use std::thread;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    thalamus::init();

    task::spawn(async {
        thalamus::p2p::init_p2p_server().await;
    });

    task::spawn(async {
        thalamus::p2p::init_p2p_client().await;
    });

    thread::spawn(|| {
 

        let mut thalamus = thalamus::ThalamusClient::load().unwrap();
        thalamus.discover();
        thalamus.save();
        log::warn!("thalamus: {:?}", thalamus);
    });

    loop{}

    Ok(())
}


