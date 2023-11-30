// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

// TODO (0.0.2):
// - OpenTTS support (DONE)
// - Yolov7 https://github.com/PixelCoda/YoloV7.cpp (DONE)
// - Configurable web pool size, port, etc. (DONE)
// - capablities framework for nodes (WIP)
// - Nural Style Transfer (WIP)
// - Yolov3 Darknet Support (WIP)
// - Move llama to 7B only by default, allow enableing 13B, 30B, 65B via the API (WIP)
// - Encrypted wav/response support 
// - Ability to opt-in to send training data to the Open Sam Foundation
// - rust bert support (translation, GPT, summerization)
// - Automatic updates

// TODO: Jobs
// - Clear local jobs and inform p2p network to clear them on server boot
// - Update p2p network with new jobs as they are created and completed
// - Use job to wrap calculate_stats, nodex, llama, stt, etc.

// Feature List
// - TTS speech synthesis using OpenTTS
// - STT speech decoding using whisper.cpp with visual wav file generation
// - Llama.cpp GPT chat generation with multiple model support (7B, 13B, 30B, 65B)
// - Nueral Style Transfer (NST)
// - Image Super Resolution Using SRGAN
// - Web API for easy intergration into existing projects
// - p2p mesh networking for optimal node selection in production enviroments
// - YoloV8 Image Recognition (WIP)
// - Who.io facial recognition (WIP)
// - SPREC speech recognition (WIP)


// use std::error::Error;
use tokio::task;
use rouille::Server;
use rouille::Response;
use simple_dns::{Name, CLASS, ResourceRecord, rdata::{RData, A, SRV}};
use simple_mdns::sync_discovery::SimpleMdnsResponder;
use std::{net::IpAddr};
use std::sync::Arc;
use std::sync::Mutex;
use local_ip_address::list_afinet_netifas;
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
use clap::Parser;
use std::io::Write;

pub fn init_log(path: String) -> Result<(), std::io::Error>{
    let mut output = std::fs::File::create(path.as_str())?;
    write!(output, "")?;
    Ok(())
}


#[tokio::main]
async fn main() {

    // Escelate to sudo, setup logging, etc.
    clearscreen::clear().unwrap();
    sudo::with_env(&["LIBTORCH", "LD_LIBRARY_PATH", "PG_DBNAME", "PG_USER", "PG_PASS", "PG_ADDRESS"]).unwrap();
    // simple_logger::SimpleLogger::new().with_colors(true).with_level(log::LevelFilter::Info).with_timestamps(true).init().unwrap();

    init_log("/opt/thalamus/output.log".to_string()).unwrap();
    simple_logger::SimpleLogger::new().with_colors(true).with_output_file("/opt/thalamus/output.log".to_string()).init().unwrap();




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

    let args = thalamus::Args::parse();
    println!("{:?}", args);

 

    // Install Thalamus
    match std::env::current_exe() {
        Ok(exe_path) => {
            let current_exe_path = format!("{}", exe_path.display());

            if current_exe_path.as_str() != "/opt/thalamus/bin/thalamus"{
                match thalamus::thalamus::setup::install(args.clone()){
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

    // Initialize tts server
    thalamus::thalamus::services::tts::init(args.clone());

    // Setup Thalamus Client
    let thalamus = Arc::new(Mutex::new(thalamus::ThalamusClient::load(0).unwrap()));

    let thalamus_async = Arc::new(futures::lock::Mutex::new(thalamus::ThalamusClient::load(0).unwrap()));
    
    // Initialize the p2p server
    // let p2p_server = task::spawn(async {
    //     thalamus::p2p::init_p2p_server().await.unwrap();
    // });


    // let thalamus_discovery_thc = Arc::clone(&thalamus);
    // let discovery_server = task::spawn(async move{
        
    //     let mut discoverx = simple_mdns::async_discovery::ServiceDiscovery::new("a", "_thalamus._tcp.local", 10).unwrap();
    //     let mut i = 0;
    //     loop{
    //         discoverx = thalamus::mdns_discovery(Arc::clone(&thalamus_discovery_thc), discoverx).await.unwrap();
    //         // thalamus::nodex_discovery(Arc::clone(&thalamus_discovery_thc)).await;
    //         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    //         i += 1;
    //         if i > 10 {
    //             discoverx = simple_mdns::async_discovery::ServiceDiscovery::new("a", "_thalamus._tcp.local", 10).unwrap();
    //             i = 0;
    //         }
    //     }
    // });

    // MDNS Responder thread
    // let thx_port = args.www_port.clone();
    // std::thread::spawn(move || {
    //     let network_interfaces = list_afinet_netifas().unwrap();
    //     let mut responder = SimpleMdnsResponder::new(10);
    //     let srv_name = Name::new_unchecked("_thalamus._tcp.local");

    //     loop{

    //         responder.clear();
        
    //         for (_name, ip) in network_interfaces.iter() {
    //             if !ip.is_loopback() && !format!("{}", ip.clone()).contains(":") && !format!("{}", ip.clone()).ends_with(".1"){
    //                 match *ip {
    //                     IpAddr::V4(ipv4) => { 
    //                         responder.add_resource(ResourceRecord::new(
    //                             srv_name.clone(),
    //                             CLASS::IN,
    //                             10,
    //                             RData::A(A { address: ipv4.into() }),
    //                         ));
    //                     },
    //                     IpAddr::V6(_ipv6) => { /* handle IPv6 */ }
    //                 }

                    
    //             }
    //         }
        
    //         responder.add_resource(ResourceRecord::new(
    //             srv_name.clone(),
    //             CLASS::IN,
    //             10,
    //             RData::SRV(SRV {
    //                 port: thx_port,
    //                 priority: 0,
    //                 weight: 0,
    //                 target: srv_name.clone()
    //             })
    //         ));

    //         std::thread::sleep(std::time::Duration::from_secs(10));

    //     }

            
        
    // });

    // Main Thread
    let main_thc = Arc::clone(&thalamus);
    let www_port = args.www_port.clone();
    let max_threads = args.max_threads.clone();
    std::thread::spawn(move || {
        match std::env::current_exe() {
            Ok(exe_path) => {
                let current_exe_path = format!("{}", exe_path.display());
                let main_sub_thc = Arc::clone(&main_thc);
                if current_exe_path.as_str() == "/opt/thalamus/bin/thalamus"{
                    let server = Server::new(format!("0.0.0.0:{}", www_port).as_str(), move |request| {
                        match thalamus::thalamus::http::handle(request, Arc::clone(&main_sub_thc)){
                            Ok(request) => {
                                log::info!("HTTP: {:?}", request);
                                return request;
                            },
                            Err(err) => {
                                log::error!("HTTP_ERROR: {}", err);
                                return Response::empty_404();
                            }
                        }
                    }).unwrap().pool_size(max_threads.into());
                
                    loop {
                        server.poll();
                    }
                } 
            },
            Err(e) => log::error!("failed to get current exe path: {e}"),
        }
    });



    
    // let _idk = tokio::join!(
    //     p2p_server,
    //     discovery_server,
    // );
    loop {}

}


