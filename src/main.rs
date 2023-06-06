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

use std::sync::Arc;
use rouille::Server;
use rouille::Response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Escelate to sudo, setup logging, etc.
    thalamus::preinit();

    // Respond to mDNS queries with thalamus service information
    thalamus::start_mdns_responder().await;



    // Initialize the p2p server
    task::spawn(async {
        thalamus::p2p::init_p2p_server().await;
    });

    // task::spawn(async {
    //     thalamus::p2p::init_p2p_client().await;
    // });

    let mut thalamus = thalamus::ThalamusClient::load().unwrap();
    

    match thalamus::thalamus::setup::install_client(){
        Ok(_) => log::warn!("Installed thalamus client"),
        Err(e) => log::error!("Error installing thalamus client: {}", e),
    };

    match std::env::current_exe() {
        Ok(exe_path) => {
            let current_exe_path = format!("{}", exe_path.display());

            if current_exe_path.as_str() == "/opt/thalamus/bin/thalamus"{
                let nodex = Arc::clone(&thalamus.nodes);
                let server = Server::new("0.0.0.0:8050", move |request| {
                    let nodey = Arc::clone(&nodex);
                    match thalamus::thalamus::http::handle(request, nodey){
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
            } else {
                match thalamus::thalamus::setup::install(){
                    Ok(_) => log::warn!("Installed thalamus"),
                    Err(e) => log::error!("Error installing thalamus: {}", e),
                };
            }
        },
        Err(e) => log::error!("failed to get current exe path: {e}"),
    };



    let mut discoverx = simple_mdns::async_discovery::ServiceDiscovery::new("a", "_thalamus._tcp.local", 10).unwrap();
    loop{
        discoverx = thalamus.mdns_discovery(discoverx).await.unwrap();
        // std::thread::sleep(std::time::Duration::from_millis(60000));
        // thalamus.ipv4_discovery();
        // thalamus.save();
    }

    Ok(())
}


