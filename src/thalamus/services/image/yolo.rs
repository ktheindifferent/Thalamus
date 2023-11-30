// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

// Yolov7 https://github.com/PixelCoda/YoloV7.cpp
// Yolo9k CoreML https://github.com/seph14/Cinder-Yolo9k/tree/master

// use std::fs;
use std::fs::File;
use std::io::{Write};
use rouille::Request;
use rouille::Response;
use rouille::input::post::BufferedFile;
use rouille::post_input;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::{Command, Stdio};
use serde::{Serialize, Deserialize};


pub fn yolov7(file_path: String) -> Result<String, String> {

    #[cfg(target_os = "linux")]{
        let child = Command::new("/opt/thalamus/bin/yolov7")
        .arg(file_path)
        .arg("/opt/thalamus/models/yolov7.onnx")
        .arg("640")
        .arg("640")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    
        let output = child
            .wait_with_output()
            .expect("failed to wait on child");
        let yolo = String::from_utf8_lossy(&output.stdout).to_string().replace("\n", "");
    
        if yolo.to_lowercase().contains("error") || yolo.len() == 0 {
            return Err(format!("yolo_v7_error: {}", yolo))
        }

      

    
        return Ok(yolo);
    }

    #[cfg(target_os = "macos")]{
        let child = Command::new("/opt/thalamus/bin/yolov7")
        .arg(file_path)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    
        let output = child
            .wait_with_output()
            .expect("failed to wait on child");
        let yolo = String::from_utf8_lossy(&output.stdout).to_string().replace("\n", "");
    
        if yolo.to_lowercase().contains("error") || yolo.len() == 0 {
            return Err(format!("yolo_v7_error: {}", yolo))
        }
    
        return Ok(yolo);
    }

    // return Err(format!("yolo_v7_error: failed to select a valid operating system"));


}

pub fn install() -> Result<(), crate::thalamus::setup::Error> {


    // Linux
    #[cfg(all(target_arch = "x86_64", target_os = "linux"))] {
        if !Path::new("/opt/thalamus/bin/yolov7").exists(){
            log::info!("Installing yolov7 (x86_64) /opt/thalamus/bin/yolov7");
            crate::thalamus::tools::safe_download(
                "/opt/thalamus/bin/yolov7", 
                "https://www.dropbox.com/s/rc4v0zpxoze6i4s/yolov7?dl=1", 
                None, 
                None
            );
        }

        if !Path::new("/opt/thalamus/models/yolov7.onnx").exists(){
            log::info!("Downloading yolov7 model /opt/thalamus/models/yolov7.onnx");
            crate::thalamus::tools::safe_download(
                "/opt/thalamus/models/yolov7.onnx", 
                "https://www.dropbox.com/s/yaxcikpiq9v6i1d/yolov7.onnx?dl=1", 
                None, 
                None
            );
        }

        match crate::thalamus::tools::mark_as_executable("/opt/thalamus/bin/yolov7"){
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod yolov7").into())
        }
    }

    // Apple M1/M2
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
        crate::thalamus::tools::safe_download(
            "/opt/thalamus/bin/yolov7", 
            "https://www.dropbox.com/s/0rj0shcmctiy6n6/yolov7?dl=1", 
            Some("4abbd78cf05ab703b99b3d984b893f2525b7045c37dc8454773aaa15e92a7bcd"), 
            Some(3153216)
        );

        crate::thalamus::tools::safe_download(
            "/opt/thalamus/bin/yolov7.mlmodelc.tar.xz", 
            "https://www.dropbox.com/s/3wm6tgv9w7d3iyp/yolov7.mlmodelc.tar.xz?dl=1", 
            Some("8364636d8863f73eeaa344fbb5547102f3b8e22c0c73574678d8d1f8303f069c"), 
            Some(111855848)
        );
    
        match crate::thalamus::tools::untar("/opt/thalamus/bin/yolov7.mlmodelc.tar.xz", "/opt/thalamus/bin/"){
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to extract coreML model for yolov7").into())
        }
        

        match crate::thalamus::tools::mark_as_executable("/opt/thalamus/bin/yolov7"){
            Ok(_) => (),
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod yolov7").into())
        }
    }

    Ok(())
}


pub fn handle(request: &Request) -> Result<Response, crate::thalamus::http::Error> {
    
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    let input = post_input!(request, {
        image_file: BufferedFile,
    })?;

    let tmp_file_path = format!("/opt/thalamus/tmp/{}.jpg", timestamp.clone());
    let mut file = File::create(tmp_file_path.clone())?;
    file.write_all(&input.image_file.data)?;
    
    if request.url() == "/api/services/image/yolo/v7" {
        let yolo = yolov7(tmp_file_path)?;
        let reply: YoloV7Output = serde_json::from_str(&yolo)?;
        return Ok(Response::json(&reply));
    }
    
    return Ok(Response::empty_404());
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoloV7Output {
    pub objects: Vec<Yolov7Object>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Yolov7Object {
    #[serde(rename = "class_id")]
    pub class_id: i64,
    pub name: String,
    pub coordinates: Coordinates,
    pub confidence: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
}
