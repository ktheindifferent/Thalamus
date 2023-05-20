// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use tch::{Device};

use std::thread;
use rouille::Request;
use rouille::Response;
use rouille::post_input;
use rouille::session;
use error_chain::error_chain;
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        // Postgres(postgres::Error);
        PostError(rouille::input::post::PostError);
        // RustTubeError(rustube::Error);
        InternalServiceError(crate::thalamus::services::Error);
        // SamMemoryError(crate::sam::memory::Error);
    }
}

pub fn init(){
    thread::spawn(move || {
        rouille::start_server(format!("0.0.0.0:8050").as_str(), move |request| {
        
            match crate::thalamus::http::handle(request){
                Ok(request) => {
                    return request;
                },
                Err(err) => {
                    log::error!("HTTP_ERROR: {}", err);
                    return Response::empty_404();
                }
            }

        });
    });
}



// TODO - Authenticate connections using a one time key and expiring Sessions
// WW
pub fn handle(request: &Request) -> Result<Response> {

    if request.url().contains("/api/services/llama"){
        return Ok(crate::thalamus::services::llama::handle(request)?);
    }

    if request.url().contains("/api/services/whisper"){
        return Ok(crate::thalamus::services::llama::handle(request)?);
    }

    if request.url().contains("/is_cuda"){
        let device = tch::Cuda::is_available();
        return Ok(Response::text(device.to_string()));
    }

    if request.url().contains("/is_cuda2"){
        let device = tch::Cuda::cudnn_is_available();
        return Ok(Response::text(device.to_string()));
    }

    if request.url().contains("/cudac"){
        let device = tch::Cuda::device_count();
        return Ok(Response::text(device.to_string()));
    }

    return Ok(Response::empty_404());
}
