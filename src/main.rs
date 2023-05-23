// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

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


