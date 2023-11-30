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

use std::path::Path;

pub fn handle(_request: &Request) -> Result<Response, crate::thalamus::http::Error> {
    return Ok(Response::empty_404());
}

use tract_tensorflow::prelude::*;

pub fn execc() -> TractResult<()> {
    let model = tract_tensorflow::tensorflow()
        // load the model
        .model_for_path("/opt/thalamus/models/ocnn/mobilenet_v2_1.4_224_frozen.pb")?
        // specify input type and shape
        .with_input_fact(0, f32::fact([1, 224, 224, 3]).into())?
        // optimize the model
        .into_optimized()?
        // make the model runnable and fix its inputs and outputs
        .into_runnable()?;

    // open image, resize it and make a Tensor out of it
    let image = image::open("/opt/thalamus/models/ocnn/grace_hopper.jpg").unwrap().to_rgb8();
    let resized =
        image::imageops::resize(&image, 224, 224, ::image::imageops::FilterType::Triangle);
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, 224, 224, 3), |(_, y, x, c)| {
        resized[(x as _, y as _)][c] as f32 / 255.0
    })
    .into();

    // run the model on the input
    let result = model.run(tvec!(image.into()))?;

    // find and display the max value with its index
    let best = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(1..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    log::info!("result: {best:?}");
    Ok(())
}

pub fn install() -> Result<(), crate::thalamus::setup::Error> {


    // Download mobilenetv2 Open Sam Foundation (OSF)
    crate::thalamus::tools::safe_download(
        "/opt/thalamus/models/ocnn/mobilenet_v2_1.4_224_frozen.pb", 
        "https://www.dropbox.com/scl/fi/heoxedvxhh033hfwmck1w/mobilenet_v2_1.4_224_frozen.pb?rlkey=q8hou6tytx6dpzvkgtz7gdsoa&dl=1", 
        Some("111479258f3841c93d0a7a377c976c24e8281077818991931429d2277dd88590"), 
        Some(24508794)
    );

    crate::thalamus::tools::safe_download(
        "/opt/thalamus/models/ocnn/imagenet_slim_labels.txt",
        "https://www.dropbox.com/scl/fi/lsbmxydwjt3xw85w0r8ew/imagenet_slim_labels.txt?rlkey=0lkio7z653um7sa14494jejc6&dl=1", 
        Some("e8d2cef25bb7b3c8c6923ad3c463b47de8b8535cadf4bd62a2ca2532c587eb9f"), 
        Some(10479)
    );

    crate::thalamus::tools::safe_download(
        "/opt/thalamus/models/ocnn/grace_hopper.jpg", 
        "https://www.dropbox.com/scl/fi/pigjncag76xm9wf6g1tsf/grace_hopper.jpg?rlkey=jqt9pnhg1ovsz22vqiq05re7b&dl=1", 
        Some("e1f57e98cf38076c0f9a058d74ffddf90f20453e436033784606b63c8ed2e49a"), 
        Some(73746)
    );


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
