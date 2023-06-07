// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

// https://github.com/suno-ai/bark

use rouille::Request;
use rouille::Response;
use rouille::post_input;
use std::path::Path;

pub fn handle(request: &Request) -> Result<Response, crate::thalamus::http::Error> {
    
    if request.url().contains("/api/services/tts"){

        let input = post_input!(request, {
            prompt: String, // Hello World!
        })?;

        match crate::thalamus::tools::tts(input.model.as_str(), input.prompt.as_str()){
            Ok(output) => {
                return Ok(Response::text(output));
            },
            Err(e) => {
                return Err(e.into());
            }
        }


    }
    
    
    return Ok(Response::empty_404());
}