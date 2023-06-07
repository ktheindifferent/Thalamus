// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

// Super Resoulution using a Generative Adversarial Network
// Makes pngs and jpegs look nice.....computer enhance image!


use rouille::Request;
use rouille::Response;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use rouille::post_input;
use rouille::input::post::BufferedFile;

pub fn handle(request: &Request) -> Result<Response, crate::thalamus::http::Error> {
    if request.url().contains("/api/services/image/srgan"){

        let input = post_input!(request, {
            filename: String,
            input_file: BufferedFile,
        })?;

        let mime_type = crate::thalamus::tools::find_mimetype(&input.filename.clone());

        let tmp_file_path = format!("/opt/thalamus/tmp/srgan/{}", input.filename.clone());
        let out_file_path = format!("/opt/thalamus/tmp/srgan/SRGAN_{}", input.filename.clone());
        let mut file = File::create(tmp_file_path.clone())?;
        file.write_all(&input.input_file.data)?;

        match crate::thalamus::tools::srgan(tmp_file_path.as_str(), out_file_path.clone().as_str()){
            Ok(_) => {

                let outfile = File::open(out_file_path.as_str()).unwrap();

                let response = Response::from_file(mime_type, outfile);
                return Ok(response);
            },
            Err(e) => {
                log::error!("{:?}", e);
            }
        }

        return Ok(Response::empty_404());


    }
    return Ok(Response::empty_404());
}

pub fn install() -> Result<(), crate::thalamus::setup::Error> {
    #[cfg(all(target_arch = "x86_64", target_os = "linux"))] {
        log::info!("Unpacking SRGAN");
        let data = include_bytes!("../../../../packages/srgan/linux/amd64/srgan");
        let mut pos = 0;
        let mut buffer = File::create("/opt/thalamus/bin/srgan")?;
        while pos < data.len() {
            let bytes_written = buffer.write(&data[pos..])?;
            pos += bytes_written;
        }
    }

    #[cfg(all(target_arch = "aarch64", target_os = "linux"))] {
        log::info!("Unpacking SRGAN");
        let data = include_bytes!("../../../../packages/srgan/linux/aarch64/srgan");
        let mut pos = 0;
        let mut buffer = File::create("/opt/thalamus/bin/srgan")?;
        while pos < data.len() {
            let bytes_written = buffer.write(&data[pos..])?;
            pos += bytes_written;
        }
    }

    #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
        log::info!("Unpacking SRGAN");
        let data = include_bytes!("../../../../packages/srgan/apple/aarch64/srgan");
        let mut pos = 0;
        let mut buffer = File::create("/opt/thalamus/bin/srgan")?;
        while pos < data.len() {
            let bytes_written = buffer.write(&data[pos..])?;
            pos += bytes_written;
        }
    }

    Ok(())
}
