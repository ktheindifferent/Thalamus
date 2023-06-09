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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionHeader {
    pub version: String,
    pub pid: String,
}


pub fn handle(request: &Request) -> Result<Response> {

    if request.url().contains("/api/thalamus/version"){
        let pid = std::fs::read_to_string("/opt/thalamus/pid").expect("Unable to read file");
        return Ok(Response::json(&VersionHeader{version: VERSION.ok_or("UNKNOWN")?.to_string(), pid: pid}));
    }

    if request.url().contains("/api/services/image"){
        return Ok(crate::thalamus::services::image::handle(request)?);
    }

    if request.url().contains("/api/nodex"){
        let thalamus = crate::ThalamusClient::load(0).unwrap();
        return Ok(Response::json(&thalamus.nodes));
    }

    if request.url().contains("/api/services/llama"){
        return Ok(crate::thalamus::services::llama::handle(request)?);
    }

    if request.url().contains("/api/services/whisper"){
        return Ok(crate::thalamus::services::whisper::handle(request)?);
    }


    return Ok(Response::html(format!("<pre> 
████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████
   ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██     
   ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████
   ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██
   ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████
   </pre>
    <p>Version: {}</p>
    ", VERSION.ok_or("UNKNOWN")?.to_string())));
}
