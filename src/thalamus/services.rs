// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.
pub mod llama;
pub mod whisper;
pub mod image;

use error_chain::error_chain;
error_chain! {
    foreign_links {
        Io(std::io::Error);
        PostError(rouille::input::post::PostError);
        InternalToolsError(crate::thalamus::tools::Error);
        // Postgres(postgres::Error);
        // PostError(rouille::input::post::PostError);
        // RustTubeError(rustube::Error);
        // InternalServiceError(crate::sam::services::Error);
        // SamMemoryError(crate::sam::memory::Error);
    }
}