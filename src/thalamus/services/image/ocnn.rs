// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

// Organic Classification Nural Network (OCNN)

// Input -> {Animal, Plant, Fungi}
// Fungi -> {Amanti, Muscare}

use rouille::Request;
use rouille::Response;
use std::fs::File;
use std::io::Write;

pub fn handle(_request: &Request) -> Result<Response, crate::thalamus::http::Error> {
    return Ok(Response::empty_404());
}

pub fn install() -> Result<(), crate::thalamus::setup::Error> {

    // log::info!("Unpacking OCNN: birds.tflite");
    // let data = include_bytes!("../../../../packages/ocnn/birds/birds.tflite");
    // let mut pos = 0;
    // let mut buffer = File::create("/opt/thalamus/models/ocnn/birds.tflite")?;
    // while pos < data.len() {
    //     let bytes_written = buffer.write(&data[pos..])?;
    //     pos += bytes_written;
    // }

    // log::info!("Unpacking OCNN: birds.txt");
    // let data = include_bytes!("../../../../packages/ocnn/birds/birds.txt");
    // let mut pos = 0;
    // let mut buffer = File::create("/opt/thalamus/models/ocnn/birds.txt")?;
    // while pos < data.len() {
    //     let bytes_written = buffer.write(&data[pos..])?;
    //     pos += bytes_written;
    // }

    Ok(())
}
