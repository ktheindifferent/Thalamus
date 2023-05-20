// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use std::path::Path;
use std::fs::File;
use std::io::Write;

// TODO: Compile whisper for raspi and patch installer
pub fn install() -> std::io::Result<()> {
    
    match crate::thalamus::tools::cmd(format!("mkdir /opt")){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt directory")),
    }

    match crate::thalamus::tools::cmd(format!("mkdir /opt/thalamus")){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus directory")),
    }

    match crate::thalamus::tools::cmd(format!("chown 1000 -R /opt/thalamus")){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chown /opt/thalamus")),
    }

    match crate::thalamus::tools::cmd(format!("mkdir /opt/thalamus/models")){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/models directory")),
    }

    match crate::thalamus::tools::cmd(format!("mkdir /opt/thalamus/bin")){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/bin directory")),
    }

    match crate::thalamus::tools::cmd(format!("mkdir /opt/thalamus/tmp")){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/tmp directory")),
    }

    match crate::thalamus::tools::cmd(format!("mkdir /opt/thalamus/fonts")){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/fonts directory")),
    }


   // Apple M1/M2
   #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
        match crate::thalamus::tools::cmd(format!("/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"")){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install homebrew")),
        }
   }


    match crate::thalamus::services::whisper::install(){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install whisper")),
    }

    
    Ok(())
}



