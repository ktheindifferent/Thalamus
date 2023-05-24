// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.





// TODO: Compile whisper for raspi and patch installer
pub fn install() -> std::io::Result<()> {
    
    match crate::thalamus::tools::mkdir("/opt"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt directory")),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus directory")),
    }

    match crate::thalamus::tools::fix_permissions("/opt/thalamus"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod /opt/thalamus")),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus/models"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/models directory")),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus/bin"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/bin directory")),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus/tmp"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/tmp directory")),
    }

    match crate::thalamus::tools::mkdir("/opt/thalamus/fonts"){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus/fonts directory")),
    }


   // Apple M1/M2
   #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {

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


        // Uninstall python
        match crate::thalamus::tools::brew_uninstall("python"){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to uninstall python")),
        }
   }


    match crate::thalamus::services::whisper::install(){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install whisper")),
    }

    match crate::thalamus::setup::install_service(){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install thalamus as a service")),
    }

    Ok(())
}



pub fn install_service() -> std::io::Result<()> {


    match std::env::current_exe() {
        Ok(exe_path) => {
            let current_exe_path = format!("{}", exe_path.display());
            match crate::thalamus::tools::cp(current_exe_path.as_str(), "/opt/thalamus/bin"){
                Ok(_) => {},
                Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to copy thalamus binary")),
            }
        },
        Err(e) => log::error!("failed to get current exe path: {e}"),
    };



    // Mac M1/M2
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
        update_osx_service_file();
        crate::thalamus::tools::launchd_bootstrap("/Library/LaunchDaemons/com.opensamfoundation.thalamus.plist");
        crate::thalamus::tools::launchd_enable("system/com.opensamfoundation.thalamus.plist");
        crate::thalamus::tools::launchd_kickstart("system/com.opensamfoundation.thalamus.plist");
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
    data.push_str("<dict>\n");
    data.push_str("<key>SuccessfulExit</key>\n");
    data.push_str("<false/>\n");
    data.push_str("</dict>\n");
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