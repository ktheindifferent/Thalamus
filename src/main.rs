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
// - internal library (DONE)
// - Complete p2p support with nodex exchange (DONE)
// - publish 0.0.1

// TODO (0.0.2):
// - Fix mdns responder first boot work bug



// - capablities framework for nodes
// - apple natice TTS support?
// - deepspeech TTS support?
// - IBM Watson TTS support?
// - Configurable web pool size, port, etc.
// - Automatic updates via the Open Sam Foundation
// - Nural Style Transfer using ANE
// - Speech Synthesis using ANE
// - Who.io using ANE?
// - Encrypted wav/response support?
// - HTTP/S encryption support?
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



// use std::error::Error;
use tokio::task;
use rouille::Server;
use rouille::Response;
use tokio::task::yield_now;
use simple_dns::{Name, CLASS, ResourceRecord, rdata::{RData, A, SRV}};
use simple_mdns::sync_discovery::SimpleMdnsResponder;
use std::{net::IpAddr};
use local_ip_address::list_afinet_netifas;
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");



#[tokio::main]
async fn main() {

    // Escelate to sudo, setup logging, etc.
    clearscreen::clear().unwrap();
    sudo::with_env(&["LIBTORCH", "LD_LIBRARY_PATH", "PG_DBNAME", "PG_USER", "PG_PASS", "PG_ADDRESS"]).unwrap();
    simple_logger::SimpleLogger::new().with_colors(true).with_level(log::LevelFilter::Info).with_utc_timestamps().init().unwrap();

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

    // Install Thalamus
    match std::env::current_exe() {
        Ok(exe_path) => {
            let current_exe_path = format!("{}", exe_path.display());

            if current_exe_path.as_str() != "/opt/thalamus/bin/thalamus"{
                match thalamus::thalamus::setup::install(){
                    Ok(_) => log::warn!("Installed thalamus"),
                    Err(e) => log::error!("Error installing thalamus: {}", e),
                };
                match thalamus::thalamus::setup::install_client(){
                    Ok(_) => log::warn!("Installed thalamus client"),
                    Err(e) => log::error!("Error installing thalamus client: {}", e),
                };
            }
        },
        Err(e) => log::error!("Error getting current executable path: {}", e),
    };


    // // Setup Thalamus Client
    // let mut thalamus = thalamus::ThalamusClient::load(0).unwrap();
    
    // // Respond to mDNS queries with thalamus service information
    // thalamus.start_mdns_responder().await;



    // Initialize the p2p server
    let p2p_server = task::spawn(async {
        thalamus::p2p::init_p2p_server().await.unwrap();
    });



    let discovery_server = task::spawn(async {
        
        let mut discoverx = simple_mdns::async_discovery::ServiceDiscovery::new("a", "_thalamus._tcp.local", 10).unwrap();
        let mut i = 0;
        loop{
            let mut thalamus = thalamus::ThalamusClient::load(0).unwrap();

            discoverx = thalamus.mdns_discovery(discoverx).await.unwrap();
            thalamus.nodex_discovery().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            i += 1;
            if i > 4 {
                discoverx = simple_mdns::async_discovery::ServiceDiscovery::new("a", "_thalamus._tcp.local", 10).unwrap();
                i = 0;
            }
        }
    });

    std::thread::spawn(|| {
        let network_interfaces = list_afinet_netifas().unwrap();
        let mut responder = SimpleMdnsResponder::new(10);
        let srv_name = Name::new_unchecked("_thalamus._tcp.local");

        loop{

            responder.clear();
        
            for (_name, ip) in network_interfaces.iter() {
                if !ip.is_loopback() && !format!("{}", ip.clone()).contains(":") && !format!("{}", ip.clone()).contains(".0.1"){
                    match *ip {
                        IpAddr::V4(ipv4) => { 
                            responder.add_resource(ResourceRecord::new(
                                srv_name.clone(),
                                CLASS::IN,
                                10,
                                RData::A(A { address: ipv4.into() }),
                            ));
                        },
                        IpAddr::V6(_ipv6) => { /* handle IPv6 */ }
                    }

                    
                }
            }
        
            responder.add_resource(ResourceRecord::new(
                srv_name.clone(),
                CLASS::IN,
                10,
                RData::SRV(SRV {
                    port: 8050,
                    priority: 0,
                    weight: 0,
                    target: srv_name.clone()
                })
            ));

            std::thread::sleep(std::time::Duration::from_secs(2));

        }

            
        
    });

    std::thread::spawn(|| {
        match std::env::current_exe() {
            Ok(exe_path) => {
                let current_exe_path = format!("{}", exe_path.display());

                if current_exe_path.as_str() == "/opt/thalamus/bin/thalamus"{
                    let server = Server::new("0.0.0.0:8050", move |request| {
                        match thalamus::thalamus::http::handle(request){
                            Ok(request) => {
                                log::info!("{:?}", request);
                                return request;
                            },
                            Err(err) => {
                                log::error!("HTTP_ERROR: {}", err);
                                return Response::empty_404();
                            }
                        }
                    }).unwrap().pool_size(3);
                
                    loop {
                        server.poll();
                    }
                } 
            },
            Err(e) => log::error!("failed to get current exe path: {e}"),
        }
    });

    // mdns
    // std::thread::spawn(|| {
    //     let responder = libmdns::Responder::new().unwrap();
    //     let _svc = responder.register(
    //         "_thalamus._tcp.local".to_owned(),
    //         "thalamus".to_owned(),
    //         8050,
    //         &["path=/"],
    //     );
    
    //     loop {
    //         ::std::thread::sleep(::std::time::Duration::from_secs(10));
    //     }
    // });

    
    let _idk = tokio::join!(
        p2p_server,
        discovery_server,
    );
    loop {}

}


