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
    
    match crate::thalamus::tools::cmd(format!("mkdir /opt")){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt directory")),
    }

    match crate::thalamus::tools::cmd(format!("mkdir /opt/thalamus")){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create /opt/thalamus directory")),
    }

    match crate::thalamus::tools::cmd(format!("chmod -R 777 /opt/thalamus")){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to chmod /opt/thalamus")),
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

        // Install Homebrew
        match crate::thalamus::tools::cmd(format!("/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"")){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install homebrew")),
        }

        // Install Miniconda
        match crate::thalamus::tools::cmd(format!("brew install miniconda")){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install miniconda")),
        }

        // Install openssl@1.1
        match crate::thalamus::tools::cmd(format!("brew install openssl@1.1")){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install openssl@1.1")),
        }

        // Install ffmpeg
        match crate::thalamus::tools::cmd(format!("brew install ffmpeg")){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install ffmpeg")),
        }

        // Uninstall python
        match crate::thalamus::tools::cmd(format!("brew uninstall python")){
            Ok(_) => {},
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to uninstall python")),
        }
   }


    match crate::thalamus::services::whisper::install(){
        Ok(_) => {},
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to install whisper")),
    }


    // TODO - Service bootstrap for mac
    // sudo launchctl bootstrap system /Library/LaunchDaemons/${YOUR_SERVICE_NAME}.plist
    // sudo launchctl enable system/${YOUR_SERVICE_NAME}
    // sudo launchctl kickstart -kp system/${YOUR_SERVICE_NAME}
    
    Ok(())
}



