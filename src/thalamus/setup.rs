// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

// TODO: sudo apt install libclblast-dev
// sudo apt-get install libopenblas-dev
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::path::Path;

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Hound(hound::Error);
        ToolKitError(crate::thalamus::tools::Error);
    }
}


// TODO: Compile whisper for raspi and patch installer
pub fn install() -> Result<()> {
    
    match crate::thalamus::tools::mkdir("/opt"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt directory").into()),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus directory").into()),
    }

    match crate::thalamus::tools::fix_permissions("/opt/thalamus"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod /opt/thalamus").into()),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus/models"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/models directory").into()),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus/models/llama"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/models/llama directory").into()),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus/models/llama/7B"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/models/llama directory").into()),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus/models/llama/13B"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/models/llama directory").into()),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus/models/ocnn"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/models/ocnn directory").into()),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus/bin"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/bin directory").into()),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus/tmp"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/tmp directory").into()),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus/fonts"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/fonts directory").into()),
    }


   // Apple M1/M2
   #[cfg(all(target_os = "macos"))] {

        // Install Homebrew
        match crate::thalamus::tools::dbash("\"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install homebrew")),
        }

        // Install Miniconda
        match crate::thalamus::tools::brew_install("miniconda"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install miniconda")),
        }

        // Install openssl@1.1
        match crate::thalamus::tools::brew_install("openssl@1.1"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install openssl@1.1")),
        }

        // Install wget
        match crate::thalamus::tools::brew_install("wget"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install openssl@1.1")),
        }

        // Install ffmpeg
        match crate::thalamus::tools::brew_install("ffmpeg"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install ffmpeg")),
        }
        match crate::thalamus::tools::ln("/opt/homebrew/bin/ffmpeg", "/opt/thalamus/bin/ffmpeg"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to link ffmpeg")),
        }

        match crate::thalamus::tools::ln("/opt/homebrew/bin/wget", "/opt/thalamus/bin/wget"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to link ffmpeg")),
        }



        // Uninstall python
        match crate::thalamus::tools::brew_uninstall("python"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to uninstall python")),
        }
   }



   #[cfg(all(target_os = "linux"))] {
        match crate::thalamus::tools::ln("/bin/wget", "/opt/thalamus/bin/wget"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to link ffmpeg").into()),
        }
   }




   match crate::thalamus::tools::ln("/opt/homebrew/bin/wget", "/opt/thalamus/bin/wget"){
    Ok(_) => {},
    Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to link ffmpeg").into()),
}






    match crate::thalamus::services::whisper::install(){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install whisper").into()),
    }

    match crate::thalamus::services::llama::install(){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install llama").into()),
    }

    match crate::thalamus::services::image::srgan::install(){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install srgan").into()),
    }

    match crate::thalamus::services::image::ocnn::install(){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install ocnn").into()),
    }

    match crate::thalamus::setup::install_service(){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install thalamus as a service").into()),
    }



    if !Path::new("/opt/thalamus/pid").exists() {
        let pid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        std::fs::write("/opt/thalamus/pid", pid).expect("Unable to write file");
    }

    Ok(())
}



pub fn install_service() -> Result<()> {


    match std::env::current_exe() {
        Ok(exe_path) => {
            let current_exe_path = format!("{}", exe_path.display());
            match crate::thalamus::tools::cp(current_exe_path.as_str(), "/opt/thalamus/bin"){
                Ok(_) => {},
                Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to copy thalamus binary").into()),
            }
        },
        Err(e) => log::error!("failed to get current exe path: {e}"),
    };



    // Mac OS
    #[cfg(all(target_os = "macos"))] {
        update_osx_service_file();
        match crate::thalamus::tools::launchd_bootout("/Library/LaunchDaemons/com.opensamfoundation.thalamus.plist"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to launch thalamus as a service")),
        }
        match crate::thalamus::tools::launchd_bootstrap("/Library/LaunchDaemons/com.opensamfoundation.thalamus.plist"){
            Ok(_) => {},
                        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to bootstrap thalamus as a service")),
        }
        match crate::thalamus::tools::launchd_enable("system/com.opensamfoundation.thalamus.plist"){
            Ok(_) => {},
                        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to enable thalamus as a service")),
        }
        match crate::thalamus::tools::launchd_kickstart("system/com.opensamfoundation.thalamus.plist"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to kickstart thalamus as a service")),
        }
    }


    // Linux
    #[cfg(all(target_os = "linux"))] {
        update_linux_service_file();
        match crate::thalamus::tools::systemctl_reload(){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to reload systemctl").into()),
        }
        match crate::thalamus::tools::systemctl_enable("thalamus.service"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to enable thalamus as a service").into()),
        }
        match crate::thalamus::tools::systemctl_stop("thalamus.service"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to stop thalamus as a service").into()),
        }
        match crate::thalamus::tools::systemctl_start("thalamus.service"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to start thalamus as a service").into()),
        }
    }

    Ok(())
}

pub fn update_osx_service_file(){
    let mut data = String::new();
    data.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    data.push_str("<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n");
    data.push_str("<plist version=\"1.0\">\n");
    data.push_str("<dict>\n");
    data.push_str("<key>Label</key>\n");
    data.push_str("<string>com.opensamfoundation.thalamus</string>\n");
    data.push_str("<key>ProgramArguments</key>\n");
    data.push_str("<array>\n");
    data.push_str("<string>/opt/thalamus/bin/thalamus</string>\n");
    data.push_str("</array>\n");

    data.push_str("<key>RunAtLoad</key>\n");
    data.push_str("<true/>\n");
    data.push_str("<key>KeepAlive</key>\n");
    data.push_str("<true/>\n");
    data.push_str("</dict>\n");
    data.push_str("</plist>\n");

    std::fs::write("/Library/LaunchDaemons/com.opensamfoundation.thalamus.plist", data).expect("Unable to write file");
}

pub fn update_linux_service_file(){
    let mut data = String::new();
    data.push_str("[Unit]\n");
    data.push_str("Description=thalamus\n");
    data.push_str("After=network.target\n");
    data.push_str("After=systemd-user-sessions.service\n");
    data.push_str("After=network-online.target\n\n");
    data.push_str("[Service]\n");
    data.push_str(format!("ExecStart=/opt/thalamus/bin/thalamus\n").as_str());
    data.push_str("TimeoutSec=30\n");
    data.push_str("Restart=on-failure\n");
    data.push_str("RestartSec=30\n");
    data.push_str("StartLimitInterval=350\n");
    data.push_str("StartLimitBurst=10\n\n");
    data.push_str("[Install]\n");
    data.push_str("WantedBy=multi-user.target\n");
    std::fs::write("/lib/systemd/system/thalamus.service", data).expect("Unable to write file");
}