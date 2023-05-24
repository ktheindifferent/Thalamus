// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.


use std::fs;
use std::io;
use std::fs::File;
use std::io::Write;
use std::path::{Path};
use std::process::{Command, Stdio};
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Hound(hound::Error);
    }
}

pub fn python3(command: String) -> String{
    let cmd = Command::new("python3")
    .arg(command.clone())
    .output()
    .unwrap();
    return String::from_utf8_lossy(&cmd.stdout).to_string();
}

// // pub fn idfk(command: &str) -> String {
//     let child = Command::new("/bin/python3")
//     .arg(command)
//     .stdout(Stdio::piped())
//     .spawn()
//     .expect("failed to execute child");

//     let output = child
//         .wait_with_output()
//         .expect("failed to wait on child");

//     return String::from_utf8_lossy(&output.stdout).to_string();
// // }

pub fn pip3(command: &str) -> String {
    let child = Command::new("/bin/pip3")
    .arg(command)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

// pub fn cmd(command: String) -> Result<String>{
//     let cmd = Command::new("sh")
//     .arg("-c")
//     .arg(command.clone())
//     .output()?;
//     return Ok(String::from_utf8_lossy(&cmd.stdout).to_string());
// }

pub fn dbash(command: &str) -> Result<String>{
    let child = Command::new("/usr/bin/sudo")
    .arg("-u")
    .arg("sam")
    .arg("/bin/bash")
    .arg("-c")
    .arg(command)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}

pub fn bash(command: &str) -> Result<String>{
    let child = Command::new("/bin/bash")
    .arg("-c")
    .arg(command)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}

pub fn brew_install(package: &str) -> Result<String>{

    let child = Command::new("/usr/bin/sudo")
    .arg("-u")
    .arg("sam")
    .arg("/opt/homebrew/bin/brew")
    .arg("install")
    .arg(package)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}

pub fn brew_uninstall(package: &str) -> Result<String>{
    let child = Command::new("/usr/bin/sudo")
    .arg("-u")
    .arg("sam")
    .arg("/opt/homebrew/bin/brew")
    .arg("uninstall")
    .arg(package)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}

pub fn ln(path: &str, link: &str) -> Result<String>{
    let child = Command::new("/bin/ln")
    .arg("-s")
    .arg(path)
    .arg(link)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}

pub fn mv(source: &str, destination: &str) -> Result<String>{
    let child = Command::new("/bin/mv")
    .arg(source)
    .arg(destination)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}

pub fn cp(source: &str, destination: &str) -> Result<String>{
    let child = Command::new("/bin/cp")
    .arg(source)
    .arg(destination)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}

pub fn launchd_bootstrap(destination: &str) -> Result<String>{
    let child = Command::new("/bin/launchctl")
    .arg("bootstrap")
    .arg("system")
    .arg(destination)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}

pub fn launchd_enable(destination: &str) -> Result<String>{
    let child = Command::new("/bin/launchctl")
    .arg("enable")
    .arg(destination)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}

pub fn launchd_kickstart(destination: &str) -> Result<String>{
    let child = Command::new("/bin/launchctl")
    .arg("kickstart")
    .arg("-kp")
    .arg(destination)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}

pub fn rm(path: &str) -> Result<String>{
    let child = Command::new("/bin/rm")
    .arg(path)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}

pub fn rmd(path: &str) -> Result<String>{
    let child = Command::new("/bin/rm")
    .arg("-rf")
    .arg(path)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}




pub fn whisper(model: &str, file_path: &str) -> Result<String>{
    
    
    let child = Command::new("/opt/thalamus/bin/whisper")
    .arg("-m")
    .arg(format!("/opt/thalamus/models/ggml-{}.bin", model))
    .arg("-f")
    .arg(format!("{}.16.wav", file_path))
    .arg("-otxt")
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
    
}

pub fn mkdir(apath: &str) -> Result<String>{
    let child = Command::new("/bin/mkdir")
    .arg(apath)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());    
}

pub fn fix_permissions(apath: &str) -> Result<String>{
    let child = Command::new("/bin/chmod")
    .arg("-R")
    .arg("777")
    .arg(apath)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string()); 
}

pub fn mark_as_executable(apath: &str) -> Result<String>{
    let child = Command::new("/bin/chmod")
    .arg("+x")
    .arg(apath)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string()); 
}

pub fn sh(script: &str) -> Result<String>{
    let child = Command::new("/bin/sh")
    .arg(script)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string()); 
}


pub fn download(file_path: &str, url: &str) -> Result<bool>{
    // let resp = reqwest::blocking::get(url)?;
    // let bytes = resp.bytes()?;
    // std::fs::write(file_path, bytes)?;

    let child = Command::new("/opt/homebrew/bin/wget")
    .arg("-O")
    .arg(file_path)
    .arg(url)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let _output = child
    .wait_with_output()
    .expect("failed to wait on child");

    // return Ok(String::from_utf8_lossy(&output.stdout).to_string()); 

    return Ok(true);
    // let mut dst = Vec::new();
    // let mut easy = Easy::new();
    // easy.url(url).unwrap();


    // {
    //     let mut transfer = easy.transfer();
    //     transfer.write_function(|data| {
    //         dst.extend_from_slice(data);
    //         Ok(data.len())
    //     }).unwrap();
    //     transfer.perform().unwrap();
    // }
    // {
    //     let mut file = std::fs::File::create(file_path)?;
    //     file.write_all(dst.as_slice())?;
    // }


    // return Ok(true);
}

pub fn wav_to_16000(input: String) -> Result<String>{
    let child = Command::new("/opt/thalamus/bin/ffmpeg")
    .arg("-y")
    .arg("-i")
    .arg(format!("{}", input))
    .arg("-ar")
    .arg("16000")
    .arg("-ac")
    .arg("1")
    .arg("-c:a")
    .arg("pcm_s16le")
    .arg(format!("{}.16.wav", input))
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");


    let output = child
    .wait_with_output()
    .expect("failed to wait on child");

    return Ok(String::from_utf8_lossy(&output.stdout).to_string());
}





pub fn touch(path: String) -> Result<()>{
    let mut output = File::create(path.as_str())?;
    write!(output, "")?;
    Ok(())
}

// subshell
// sudo -u USERNAME bash -c 'whoami;echo $USER'

// ffmpeg -i samples/ruler.mp3 samples/ruler.wav

// ffmpeg -i samples/ruler.wav -ar 16000 -ac 1 -c:a pcm_s16le samples/ruler.16.wav

// ./models/generate-coreml-model.sh large


pub fn does_wav_have_sounds(audio_filename: String) -> Result<bool>{
    let mut has_sounds = false;
	let threshold = 14000 as i16;

	let mut audio_file = hound::WavReader::open(audio_filename)?;

	let raw_samples = audio_file.samples::<i16>().into_iter().map(|x| x.unwrap()).collect::<Vec<i16>>();

	let mut samples: Vec<i16> = Vec::new();

	for i in 0..=raw_samples.len() - 1 {
		if i % 100 == 0 {
	

			if raw_samples[i as usize] > threshold || raw_samples[i as usize] < -threshold{
				has_sounds = true;
			}

			samples.push(raw_samples[i as usize]);
		}
	}

    return Ok(has_sounds);
}


pub fn extract_zip(zip_path: &str, desination_path: &str) -> std::io::Result<()> {
    let fname = std::path::Path::new(zip_path);
    let file = fs::File::open(&fname)?;

    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath_end = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let out_mend = desination_path.to_owned() + outpath_end.to_str().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))?;

        let outpath = Path::new(&(out_mend));

        if (&*file.name()).ends_with('/') {
            log::info!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            log::info!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }
    return Ok(());
}
