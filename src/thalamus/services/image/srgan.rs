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

use std::path::Path;
use rouille::Request;
use rouille::Response;
use std::fs::File;
use std::io::Write;
// use std::io::Read;
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
    if !Path::new("/opt/thalamus/bin/srgan").exists(){
        #[cfg(all(target_arch = "x86_64", target_os = "linux"))] {
            log::info!("Unpacking SRGAN");
            crate::thalamus::tools::download("/opt/thalamus/bin/srgan", "https://www.dropbox.com/s/l4smcanvwjf3huy/srgan")?;
        }

        #[cfg(all(target_arch = "aarch64", target_os = "linux"))] {
            log::info!("Unpacking SRGAN");
            crate::thalamus::tools::download("/opt/thalamus/bin/srgan", "https://www.dropbox.com/s/sgf76zwss8m4xu3/srgan")?;
        }

        #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
            log::info!("Unpacking SRGAN");
            crate::thalamus::tools::download("/opt/thalamus/bin/srgan", "https://www.dropbox.com/s/52imqf6clftie47/srgan")?;
        }
    }

    match crate::thalamus::tools::mark_as_executable("/opt/thalamus/bin/srgan"){
        Ok(_) => (),
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod whisper").into())
    }

    Ok(())
}
