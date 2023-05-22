// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.


extern crate rouille;



pub mod thalamus;

// store application version as a const
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");


fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    }


    // simple_logger::SimpleLogger::new().with_colors(true).init().unwrap();
    log::info!("VERSION: {:?}", VERSION);
    sudo::with_env(&["LIBTORCH", "LD_LIBRARY_PATH", "PG_DBNAME", "PG_USER", "PG_PASS", "PG_ADDRESS"]).unwrap();
    
    match crate::thalamus::setup::install(){
        Ok(_) => println!("Installed thalamus"),
        Err(e) => println!("Error installing thalamus: {}", e),
    };

    // let mut ctx = WhisperContext::new("/opt/whispers/models/ggml-base.en.bin").expect("failed to load model");
    // let mut state = ctx.create_state().expect("failed to create state");

    crate::thalamus::http::init();

    loop {}
}


