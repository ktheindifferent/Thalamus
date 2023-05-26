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



// TODO - Authenticate connections using a one time key and expiring Sessions
// WW
pub fn handle(request: &Request) -> Result<Response> {

    if request.url().contains("/api/services/image"){
        return Ok(crate::thalamus::services::image::handle(request)?);
    }

    if request.url().contains("/api/services/llama"){
        return Ok(crate::thalamus::services::llama::handle(request)?);
    }

    if request.url().contains("/api/services/whisper"){
        return Ok(crate::thalamus::services::whisper::handle(request)?);
    }


    return Ok(Response::empty_404());
}
