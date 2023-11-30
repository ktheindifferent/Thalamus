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

// TODO: Patch linux to 1.1 version of llama
// TODO: Add support for 13B, 30B, and 65B
pub fn install() -> Result<(), crate::thalamus::setup::Error> {

    if !Path::new("/opt/thalamus/bin/llama").exists(){
        #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
            crate::thalamus::tools::safe_download("/opt/thalamus/bin/llama", "https://www.dropbox.com/scl/fi/4ljqsqbvtwcmtqz3sr7db/main?rlkey=a0ktgg402tyoxmcyuy9fzii7k&dl=1", None, None);
        }
        #[cfg(all(target_arch = "aarch64", target_os = "linux"))] {
            crate::thalamus::tools::safe_download("/opt/thalamus/bin/llama", "https://www.dropbox.com/s/5cxh3hduwwjv0vv/main?dl=1", None, None);
        }
        #[cfg(all(target_arch = "x86_64", target_os = "linux"))] {
            crate::thalamus::tools::safe_download("/opt/thalamus/bin/llama", "https://www.dropbox.com/s/93sj2fruleo80y0/main?dl=1", None, None);
        }
    }

    match crate::thalamus::tools::mark_as_executable("/opt/thalamus/bin/llama"){
        Ok(_) => (),
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod whisper").into())
    }

    
    // Download quantized 7B llama model from Open Sam Foundation (OSF)
    crate::thalamus::tools::safe_download(
        "/opt/thalamus/models/llama/7B/ggml-model-q4_0.gguf", 
        "https://www.dropbox.com/scl/fi/6faxqth8re7dgn1ygwsbr/ggml-model-q4_0.gguf?rlkey=b1ozpsxx6nqz5f6vutva0mlz5&dl=1", 
        Some("f1c4e91ce7a6f0eaa0f4229caf473c882ad642fa7e30b4b7fb4a1377b76f6d0a"),
        Some(3825806912)
    );

    Ok(())
}




