// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.



// use std::thread;
use rouille::Request;
use rouille::Response;
use serde::{Serialize, Deserialize};

// store application version as a const
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

use error_chain::error_chain;
error_chain! {
    foreign_links {
        Io(std::io::Error);
        SystemTimeError(std::time::SystemTimeError);
        // Postgres(postgres::Error);
        PostError(rouille::input::post::PostError);
        // RustTubeError(rustube::Error);
        InternalServiceError(crate::thalamus::services::Error);
        ToolKitError(crate::thalamus::tools::Error);
        // SamMemoryError(crate::sam::memory::Error);
    }
}

// pub fn init(){
//     thread::spawn(move || {
//         rouille::start_server(format!("0.0.0.0:8050").as_str(), move |request| {
        
            

//         });
//     });
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionHeader {
    pub version: String,
}

// TODO - Authenticate connections using a one time key and expiring Sessions
// WW
pub fn handle(request: &Request) -> Result<Response> {

    if request.url().contains("/api/thalamus/version"){
        return Ok(Response::json(&VersionHeader{version: VERSION.ok_or("UNKNOWN")?.to_string()}));
    }

    if request.url().contains("/api/services/image"){
        return Ok(crate::thalamus::services::image::handle(request)?);
    }

    if request.url().contains("/api/services/llama"){
        return Ok(crate::thalamus::services::llama::handle(request)?);
    }

    if request.url().contains("/api/services/whisper"){
        return Ok(crate::thalamus::services::whisper::handle(request)?);
    }


    return Ok(Response::html(format!("
    <p> ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ </p>
    <p>    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      </p>
    <p>    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ </p>
    <p>    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ </p>
    <p>    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████ </p>
    <p>Version: {}</p>
    ", VERSION.ok_or("UNKNOWN")?.to_string())));
}
