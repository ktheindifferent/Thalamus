// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

// Full backup: ipfs://Qmb9y5GCkTG7ZzbBWMu2BXwMkzyCKcUjtEKPpgdZ7GEFKm
// 7B: ipfs://QmbvdJ7KgvZiyaqHw5QtQxRtUd7pCAdkWWbzuvyKusLGTw
// 13B: ipfs://QmPCfCEERStStjg4kfj3cmCUu1TP7pVQbxdFMwnhpuJtxk
// 30B: ipfs://QmSD8cxm4zvvnD35KKFu8D9VjXAavNoGWemPW1pQ3AF9ZZ
// 65B: ipfs://QmdWH379NQu8XoesA8AFw9nKV2MpGR4KohK7WyugadAKTh
// ipfs get QmbvdJ7KgvZiyaqHw5QtQxRtUd7pCAdkWWbzuvyKusLGTw --output ./7B

use rouille::Request;
use rouille::Response;


use rouille::post_input;

use std::path::Path;

// curl -d "prompt=tell me about abe lincoln&model=7B" -X POST http://172.16.0.15:8050/api/services/llama
pub fn handle(request: &Request) -> Result<Response, crate::thalamus::http::Error> {
    
    if request.url().contains("/api/services/llama"){

        let input = post_input!(request, {
            prompt: String, // Hello World!
            model: String, // 7B
        })?;

        match crate::thalamus::tools::llama(input.model.as_str(), input.prompt.as_str()){
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

pub fn install() -> Result<(), crate::thalamus::setup::Error> {

    if !Path::new("/opt/thalamus/bin/llama").exists(){
        #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
            crate::thalamus::tools::download("/opt/thalamus/bin/llama", "https://www.dropbox.com/s/oaim2iya4jt2l69/main")?;
        }
        #[cfg(all(target_arch = "aarch64", target_os = "linux"))] {
            crate::thalamus::tools::download("/opt/thalamus/bin/llama", "https://www.dropbox.com/s/5cxh3hduwwjv0vv/main")?;
        }
        #[cfg(all(target_arch = "x86_64", target_os = "linux"))] {
            crate::thalamus::tools::download("/opt/thalamus/bin/llama", "https://www.dropbox.com/s/93sj2fruleo80y0/main")?;
        }
    }

    match crate::thalamus::tools::mark_as_executable("/opt/thalamus/bin/llama"){
        Ok(_) => (),
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod whisper").into())
    }

    
    // Download quantized 7B llama model from Open Sam Foundation (OSF)
    if !Path::new("/opt/thalamus/models/llama/7B/ggml-model-q4_0.bin").exists(){
        log::warn!("ggml-base.bin is missing.....downloading it from https://www.dropbox.com/s/rxvd04dhxxgkfh8/ggml-model-q4_0.bin");
        match crate::thalamus::tools::download("/opt/thalamus/models/llama/7B/ggml-model-q4_0.bin", "https://www.dropbox.com/s/rxvd04dhxxgkfh8/ggml-model-q4_0.bin"){
            Ok(_) => {
                log::info!("Stored model in /opt/thalamus/models/");
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download base whisper model").into())
        }
    }


    // Download quantized 13B llama model from Open Sam Foundation (OSF)
    // https://www.dropbox.com/s/3gt8kzyw9kxc79q/ggml-model-q4_0.bin
    if !Path::new("/opt/thalamus/models/llama/13B/ggml-model-q4_0.bin").exists(){
        log::warn!("ggml-base.bin is missing.....downloading it from https://www.dropbox.com/s/3gt8kzyw9kxc79q/ggml-model-q4_0.bin");
        match crate::thalamus::tools::download("/opt/thalamus/models/llama/13B/ggml-model-q4_0.bin", "https://www.dropbox.com/s/3gt8kzyw9kxc79q/ggml-model-q4_0.bin"){
            Ok(_) => {
                log::info!("Stored model in /opt/thalamus/models/");
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download base whisper model").into())
        }
    }

    // Download quantized 30B llama model from Open Sam Foundation (OSF)
    // https://www.dropbox.com/s/3jpddk0uoghr0eo/ggml-model-q4_0.bin
    if !Path::new("/opt/thalamus/models/llama/30B/ggml-model-q4_0.bin").exists(){
        log::warn!("ggml-base.bin is missing.....downloading it from https://www.dropbox.com/s/3jpddk0uoghr0eo/ggml-model-q4_0.bin");
        match crate::thalamus::tools::download("/opt/thalamus/models/llama/30B/ggml-model-q4_0.bin", "https://www.dropbox.com/s/3jpddk0uoghr0eo/ggml-model-q4_0.bin"){
            Ok(_) => {
                log::info!("Stored model in /opt/thalamus/models/");
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download base whisper model").into())
        }
    }

    // Download quantized 65B llama model from Open Sam Foundation (OSF)
    // https://www.dropbox.com/s/ucstzvb0bzlxcyc/ggml-model-q4_0.bin
    if !Path::new("/opt/thalamus/models/llama/65B/ggml-model-q4_0.bin").exists(){
        log::warn!("ggml-base.bin is missing.....downloading it from https://www.dropbox.com/s/ucstzvb0bzlxcyc/ggml-model-q4_0.bin");
        match crate::thalamus::tools::download("/opt/thalamus/models/llama/65B/ggml-model-q4_0.bin", "https://www.dropbox.com/s/ucstzvb0bzlxcyc/ggml-model-q4_0.bin"){
            Ok(_) => {
                log::info!("Stored model in /opt/thalamus/models/");
            },
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download base whisper model").into())
        }
    }


    Ok(())
}




