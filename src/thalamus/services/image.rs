// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.
// pub mod nst;
pub mod srgan;
pub mod ocnn;
pub mod nst;
pub mod yolo;



use rouille::Request;
use rouille::Response;



pub fn install() -> Result<(), crate::thalamus::setup::Error> {
    // match nst::install(){
    //     Ok(_) => {
    //         log::info!("NST installed successfully");
    //     },
    //     Err(e) => {
    //         log::error!("Failed to install NST: {}", e);
    //     }
    // }

    Ok(())
}


pub fn handle(request: &Request) -> Result<Response, crate::thalamus::http::Error> {
    
    if request.url().contains("/api/services/image/ocnn"){
        return Ok(crate::thalamus::services::image::ocnn::handle(request)?);
    }
    
    if request.url().contains("/api/services/image/srgan"){
        return Ok(crate::thalamus::services::image::srgan::handle(request)?);
    }

    if request.url().contains("/api/services/image/nst"){
        return Ok(crate::thalamus::services::image::nst::handle(request)?);
    }

    return Ok(Response::empty_404());
}