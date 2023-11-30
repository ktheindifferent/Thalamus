// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

// use local_ip_address::list_afinet_netifas;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};


// use tokio::task;
use std::thread;
use std::sync::mpsc;

extern crate rouille;

pub mod thalamus;
pub mod p2p;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "en")]
    pub lang: String,
    #[arg(short, long, default_value_t = 6)]
    pub max_threads: u8,
    #[arg(short, long, default_value_t = 8050)]
    pub www_port: u16,
    #[arg(short, long, default_value_t = 62649)]
    pub p2p_port: u16,
    #[arg(short, long, default_value_t = false)]
    pub encrypt: bool,
    #[arg(short, long, default_value = "thalamus")]
    pub key: String,
}

pub async fn nodex_discovery(thalamus: Arc<Mutex<ThalamusClient>>){
    
    let thalamus_x = thalamus.lock().unwrap();
    let thx_clone = thalamus_x.clone();
    std::mem::drop(thalamus_x);
    
    for node in thx_clone.nodes{
        let nodexs_wrap = node.nodex();
        match nodexs_wrap {
            Ok(nodexs) => {
                for nodex in nodexs{
                    let mut thalamus_x = thalamus.lock().unwrap();

                    let existing_index = thalamus_x.nodes.clone().iter().position(|r| r.pid == nodex.pid.to_string());
                    match existing_index {
                        Some(_) => {

                        },
                        None => {
                            thalamus_x.nodes.push(nodex);
                            thalamus_x.save();
                        }
                    }
                    std::mem::drop(thalamus_x);
                }
            },
            Err(e) => {
                log::error!("nodex_discovery_error: {}", e);
            }
        }

    }
}

pub async fn mdns_discovery(thalamus: Arc<Mutex<ThalamusClient>>, discovery: simple_mdns::async_discovery::ServiceDiscovery) -> Result<simple_mdns::async_discovery::ServiceDiscovery, std::io::Error> {
    let services = discovery.get_known_services().await;
    if services.len() > 0 {
        for xy in services{
            // Register using ip address
            for ipfx in xy.ip_addresses.clone(){
                let ipx = ipfx.to_string();
                let port = xy.ports[0];
                if !ipx.to_string().ends_with(".1"){
                    let version = async_fetch_version(ipx.as_str(), port).await;
                    match version {
                        Ok(v) => {
                            let mut thalamus_x = thalamus.lock().unwrap();

               
                            let existing_index = thalamus_x.nodes.clone().iter().position(|r| r.pid == v.pid.to_string());
                            match existing_index {
                                Some(index) => {
      
                                    thalamus_x.nodes[index].is_online = true;
                                    thalamus_x.nodes[index].last_ping = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
                                    // log::info!("NODE_ONLINE: {:?}", thalamus_x.nodes[index].clone());
                                
                                    thalamus_x.save();
                                    
                                    
                                    std::mem::drop(thalamus_x);
                                },
                                None => {
                                    let v_thc = v.clone();
                               
                                        
                                    let thalamus_node = ThalamusNode::new(v_thc.pid.to_string(), v_thc.version.to_string(), ipx.clone(), port);
                                    log::info!("NEW_NODE: {:?}", thalamus_node.clone());
                                    thalamus_x.nodes.push(thalamus_node);
                                    thalamus_x.save();
                                    std::mem::drop(thalamus_x);

                                    calc_stats(Arc::clone(&thalamus), v_thc.pid.to_string(), v_thc.version.to_string(), ipx.clone(), port);
                                    
                        
                                }
                            }
                            
                        },
                        Err(e) => {
                            log::error!("fetch_thalamus_version_error: {}", e);
                            let mut thalamus_x = thalamus.lock().unwrap();
      
         
                            let existing_index = thalamus_x.nodes.clone().iter().position(|r| r.ip_address == format!("{}:{}", ipx, port).as_str());
                            match existing_index {
                                Some(index) => {

                                    thalamus_x.nodes[index].is_online = false;
                                    thalamus_x.save();
                                    log::warn!("NODE_OFFLINE: {:?}", thalamus_x.nodes[index].clone());
                       
                                    std::mem::drop(thalamus_x);
                                   
                                },
                                None => {
                                    std::mem::drop(thalamus_x);
                                }
                            }
                        }
                    }
                }
            }

            // flag missing nodes as offline
            let mut thalamus_x = thalamus.lock().unwrap();
            for node in &mut thalamus_x.nodes {

                let existing_index = xy.ip_addresses.clone().iter().position(|r| node.ip_address.contains(format!("{}", r).as_str()));
                match existing_index {
                    Some(_) => {

                    },
                    None => {
                        node.is_online = false;
                    }
                }
            }
            thalamus_x.save();
            std::mem::drop(thalamus_x);


            // flag missing nodes as offline
            // TODO: fetch fresh stats and reaverage them, and check for missing capabilities based on stats
            let thalamus_x = thalamus.lock().unwrap();
            let thx = thalamus_x.clone();
            std::mem::drop(thalamus_x);
            for node in thx.nodes {
                if !node.jobs.iter().any(|i| i.job_identifier=="calculate_stats") && node.stats.whisper_stt_tiny.is_none(){
                    log::warn!("STATS ARE MISSING FOR NODE: {:?}.....CALCULATING NOW...", node.pid.clone());
                    calc_stats(Arc::clone(&thalamus), node.pid.to_string(), node.version.to_string(), format!("{}", node.ip_address), node.port);
                }
            }
            

        }
    }

    Ok(discovery)
}

pub fn calc_stats(thalamus: Arc<Mutex<ThalamusClient>>, pid: String, version: String, ipx: String, port: u16){
    // Calculate Stats for new node
    let node_thc = Arc::clone(&thalamus);
    std::thread::spawn(move || {

        // Commit job to memory
        let job = ThalamusNodeJob::new("calculate_stats".to_string());
        let mut thalamus_x = node_thc.lock().unwrap();
        for node in &mut thalamus_x.nodes{
            if node.pid == pid.to_string(){
                node.jobs.push(job.clone());
            }
        }
        thalamus_x.save();
        std::mem::drop(thalamus_x);

        // Generate stats using dummy node data
        let node_ref = ThalamusNode::new(pid.to_string(), version.to_string(), ipx, port);
        let stats = ThalamusNodeStats::calculate(node_ref.clone());

        // Commit stats to memory
        let mut thalamus_x = node_thc.lock().unwrap();
        for node in &mut thalamus_x.nodes{
            if node.pid == pid.to_string(){
                node.stats = stats.clone();
                let index = node.jobs.iter().position(|x| *x.oid == job.oid.to_string() || *x.job_identifier == format!("calculate_stats")).unwrap();
                node.jobs.remove(index);
            }
        }
        thalamus_x.save();
        std::mem::drop(thalamus_x);
    
    });
}


/// Struct for storing all nodes connected to the client
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThalamusClient {
    pub nodes: Vec<ThalamusNode>,
}
impl ThalamusClient {
    pub fn new() -> ThalamusClient {
        let x: Vec<ThalamusNode> = Vec::new();
        ThalamusClient { 
            nodes: x,
        }
    }

    // pub fn ipv4_discovery(&mut self){

    //     let network_interfaces = list_afinet_netifas().unwrap();

    //     let nodex = Arc::clone(&self.nodes);

    //     for (name, ip) in network_interfaces.iter() {
    //         if !ip.is_loopback() && !format!("{}", ip.clone()).contains(":"){
    //             log::warn!("{}:\t{:?}", name, ip);
    //             let ips = crate::thalamus::tools::netscan::scan_bulk(format!("{}", ip).as_str(), "8050", "/24").unwrap();
    //             log::warn!("Found {} ips", ips.len());
            
    //             // Check matching ips for thalamus version info
    //             for ipx in ips{
    //                 let version = fetch_version(ipx.as_str());
    //                 match version {
    //                     Ok(v) => {
    //                         let mut nodes = nodex.lock().unwrap();
    //                         let existing_index = nodes.clone().iter().position(|r| r.pid == v.pid.to_string());
    //                         match existing_index {
    //                             Some(_index) => {
    //                             },
    //                             None => {
    //                                 nodes.push(ThalamusNode::new(v.pid.to_string(), v.version.to_string(), ipx, 8050));
    //                             }
    //                         }
    //                         std::mem::drop(nodes);
    //                     },
    //                     Err(e) => {
    //                         log::error!("fetch_thalamus_version_error: {}", e);
    //                     }
    //                 }
    //             }
                
    //         }
           
    //     }


        
    // }





    pub fn save(&self){
        std::fs::File::create("/opt/thalamus/clients.json").expect("create failed");
        let j = serde_json::to_string(&self).unwrap();
        std::fs::write("/opt/thalamus/clients.json", j).expect("Unable to write file");

        if self.nodes.len() > 0 {
            std::fs::File::create("/opt/thalamus/clients.bak.json").expect("create failed");
            let j = serde_json::to_string(&self).unwrap();
            std::fs::write("/opt/thalamus/clients.bak.json", j).expect("Unable to write file");
        }
    }

    pub fn load(retries: i64) -> Result<ThalamusClient, Box<dyn Error>>{

        if !std::path::Path::new("/opt/thalamus/clients.json").exists(){
            let new_c = ThalamusClient::new();
            new_c.save();
            return Ok(new_c);
        }

        let save_file = std::fs::read_to_string("/opt/thalamus/clients.json");
        match save_file {
            Ok(save_data) => {
                let v: Result<ThalamusClient, _> = serde_json::from_str(&save_data);
                match v {
                    Ok(v2) => {
                        return Ok(v2);
                    },
                    Err(e) => {
                        log::error!("{}", format!("Unable to parse save file: {}", e));
                        
                        if retries < 10 {
                            std::fs::copy("/opt/thalamus/clients.bak.json", "/opt/thalamus/clients.json")?;
                            std::thread::sleep(std::time::Duration::from_secs(2));
                            return Self::load(retries + 1);
                        } else {
                            log::warn!("Unable to parse save file after 10 attempts....creating new save file.");
                            let new_c = ThalamusClient::new();
                            new_c.save();
                            return Ok(new_c);
                        }
                 
                    }
                }
                
            },
            Err(e) => {
                log::error!("{}", format!("Unable to read save file: {}", e));
                if retries < 10 {
                    std::fs::copy("/opt/thalamus/clients.bak.json", "/opt/thalamus/clients.json")?;
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    return Self::load(retries + 1);
                } else {
                    log::warn!("Unable to read save file after 10 attempts....creating new save file.");
                    let new_c = ThalamusClient::new();
                    new_c.save();
                    return Ok(new_c);
                }
            }
        }
    }

    // pub fn select_optimal_node(&self, node_type: String) -> Result<ThalamusNode, Box<dyn Error + '_>> {
    //     let nodex = self.nodes.lock()?;
    //     let nodes = nodex.clone();
    //     std::mem::drop(nodex);

    //     let mut selected_node = nodes[0].clone();
    //     for node in nodes {
    //         let stats = node.stats.clone();
    //         if stats.whisper_stt_score < Some(fastest_whisper_stt_score) && node_type.contains("stt") {
    //             fastest_whisper_stt_score = stats.whisper_stt_score;
    //             selected_node = node.clone();
    //         }
    //         if stats.whisper_vwav_score < fastest_whisper_vwav_score && node_type.contains("vwav") {
    //             fastest_whisper_vwav_score = stats.whisper_vwav_score;
    //             selected_node = node.clone();
    //         }
    //         if stats.srgan < fastest_srgan_score && node_type.contains("srgan") {
    //             fastest_srgan_score = stats.srgan;
    //             selected_node = node.clone();
    //         }
    //         if stats.llama_score < fastest_llama_score && node_type.contains("llama") {
    //             fastest_llama_score = stats.llama_score;
    //             selected_node = node.clone();
    //         }
    //     }
        
    //     return Ok(selected_node);
    // }
}



pub fn fetch_version(host: &str, port: u16) -> Result<VersionReply, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    return Ok(client.get(format!("http://{}:{}/api/thalamus/version", host, port.clone())).send()?.json()?);
}

pub async fn async_fetch_version(host: &str, port: u16) -> Result<VersionReply, Box<dyn Error>> {
    let client = reqwest::Client::builder().build()?;
    return Ok(client.get(format!("http://{}:{}/api/thalamus/version", host, port.clone())).send().await?.json().await?);
}





/// Struct for storing node information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThalamusNode {
    pub pid: String,
    pub ip_address: String, // unique
    pub version: String,
    pub port: u16,
    pub jobs: Vec<ThalamusNodeJob>,
    pub capablities: Option<Vec<ThalamusNodeCapability>>,
    pub last_ping: i64,
    pub stats: ThalamusNodeStats,
    pub is_online: bool,
}
impl ThalamusNode {

    pub fn new(pid: String, version: String, ip_address: String, port: u16) -> ThalamusNode {
        let jobs: Vec<ThalamusNodeJob> = Vec::new();
        let mut node = ThalamusNode { 
            pid: pid,
            ip_address: ip_address,
            jobs: jobs,
            version: version,
            capablities: None,
            port: port,
            last_ping: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            stats: ThalamusNodeStats::new(),
            is_online: true,
        };
        let stats = ThalamusNodeStats::new();
        node.stats = stats;
        return node;
    }

    pub fn yolov7(&self, file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().file("image_file", file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}:{}/api/services/image/yolo/v7", self.ip_address.clone(), self.port.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn whisper_stt_tiny(&self, file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "tiny").file("speech", file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}:{}/api/services/whisper", self.ip_address.clone(), self.port.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn whisper_stt_base(&self, file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "basic").file("speech", file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}:{}/api/services/whisper", self.ip_address.clone(), self.port.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn whisper_stt_medium(&self, file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "medium").file("speech", file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}:{}/api/services/whisper", self.ip_address.clone(), self.port.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn whisper_stt_large(&self, file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "large").file("speech", file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}:{}/api/services/whisper", self.ip_address.clone(), self.port.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn whisper_vwav_tiny(&self, file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{
        
        let url = format!("http://{}:{}/api/services/whisper/vwav", self.ip_address.clone(), self.port.clone());

        log::info!("Fetching VWAV from {}", url);
        
        let form = reqwest::blocking::multipart::Form::new().text("method", "tiny").file("speech", file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(url)
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn whisper_vwav_base(&self, file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{
                
        let url = format!("http://{}:{}/api/services/whisper/vwav", self.ip_address.clone(), self.port.clone());

        log::info!("Fetching VWAV from {}", url);
        
        
        let form = reqwest::blocking::multipart::Form::new().text("method", "base").file("speech", file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(url)
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn whisper_vwav_medium(&self, file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{
        let url = format!("http://{}:{}/api/services/whisper/vwav", self.ip_address.clone(), self.port.clone());

        log::info!("Fetching VWAV from {}", url);
        
        let form = reqwest::blocking::multipart::Form::new().text("method", "medium").file("speech", file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(url)
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn whisper_vwav_large(&self, file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{
        let url = format!("http://{}:{}/api/services/whisper/vwav", self.ip_address.clone(), self.port.clone());

        log::info!("Fetching VWAV from {}", url);
        
        let form = reqwest::blocking::multipart::Form::new().text("method", "large").file("speech", file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(url)
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn srgan(&self, file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{

        let parts: Vec<&str> = file_path.split('.').collect();

        let extension = parts[parts.len() - 1];

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;

        let new_file_name = format!("{}.{}", timestamp, extension);

        let form = reqwest::blocking::multipart::Form::new().text("filename", new_file_name).file("input_file", file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}:{}/api/services/image/srgan", self.ip_address.clone(), self.port.clone()))
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn llama(&self, prompt: String, model: String) -> Result<String, Box<dyn Error>>{
        let params = [("model", model.as_str()), ("prompt", prompt.as_str())];

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}:{}/api/services/llama", self.ip_address.clone(), self.port.clone()))
        .form(&params)
        .send()?.text()?;

        return Ok(bytes.to_string());
    }

    pub fn tts(&self, prompt: String, primary: String, fallback: String) -> Result<Vec<u8>, Box<dyn Error>>{
        let params = [("text", prompt.as_str()), ("primary", primary.as_str()), ("fallback", fallback.as_str())];

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}:{}/api/services/tts", self.ip_address.clone(), self.port.clone()))
        .form(&params)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn nodex(&self) -> Result<Vec<ThalamusNode>, Box<dyn Error>>{
        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let mut url = format!("http://{}:{}/api/nodex", self.ip_address.clone(), self.port.clone());
        if !url.contains(":") {
            url = format!("{}:{}", url, self.port.clone());
        }

        return Ok(client.get(url)
        .send()?.json()?);
    }

    pub fn test_tts(&self) -> Result<std::option::Option<i64>, std::sync::mpsc::RecvTimeoutError>{
        log::info!("{}: Running TTS test...", self.pid);
        let (sender, receiver) = mpsc::channel();
        let node_c = self.clone();
        let _t = thread::spawn(move || {
            let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            let _tts = node_c.tts(format!("hello, my name is sam."), format!("coqui-tts:en_ljspeech"), format!("opensamfoundation")).unwrap();
            let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            let time_elapsed = Some(end_timestamp - start_timestamp);

            match sender.send(time_elapsed) {
                Ok(()) => {}, // everything good
                Err(_) => {}, // we have been released, don't panic
            }
        });
        return receiver.recv_timeout(std::time::Duration::from_millis(100));
    }

    pub fn test_yolov7(&self) -> Result<std::option::Option<i64>, std::sync::mpsc::RecvTimeoutError>{
        log::info!("{}: Running YOLOv7 test...", self.pid);
        let (sender, receiver) = mpsc::channel();
        let node_c = self.clone();
        let _t = thread::spawn(move || {
            let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            let _ = node_c.yolov7("/opt/thalamus/test.jpg".to_string()).unwrap();
            let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            let time_elapsed = Some(end_timestamp - start_timestamp);

            match sender.send(time_elapsed) {
                Ok(()) => {}, // everything good
                Err(_) => {}, // we have been released, don't panic
            }
        });
        return receiver.recv_timeout(std::time::Duration::from_millis(100));
    }


    pub fn test_srgan(&self) -> Result<std::option::Option<i64>, std::sync::mpsc::RecvTimeoutError>{
        log::info!("{}: Running SRGAN test...", self.pid);
        let (sender, receiver) = mpsc::channel();
        let node_c = self.clone();
        let _t = thread::spawn(move || {
            let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            let _srgan = node_c.srgan("/opt/thalamus/test.jpg".to_string()).unwrap();
            let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            let time_elapsed = Some(end_timestamp - start_timestamp);

            match sender.send(time_elapsed) {
                Ok(()) => {}, // everything good
                Err(_) => {}, // we have been released, don't panic
            }
        });
        return receiver.recv_timeout(std::time::Duration::from_millis(100));
    }

    pub fn test_llama(&self, model: String) -> Result<std::option::Option<i64>, std::sync::mpsc::RecvTimeoutError>{
        log::info!("{}: Running LLAMA {} test...", self.pid, model);
        let (sender, receiver) = mpsc::channel();
        let node_c = self.clone();
        let _t = thread::spawn(move || {
            let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            let _llama = node_c.llama("Tell me about Abraham Lincoln.".to_string(), model.to_string()).unwrap();
            let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            let time_elapsed = Some(end_timestamp - start_timestamp);

            match sender.send(time_elapsed) {
                Ok(()) => {}, // everything good
                Err(_) => {}, // we have been released, don't panic
            }
        });
        return receiver.recv_timeout(std::time::Duration::from_secs(100));
    }

    pub fn test_whisper_stt(&self, model: String) -> Result<std::option::Option<i64>, std::sync::mpsc::RecvTimeoutError>{
        log::info!("{}: Running Whisper STT {} test...", self.pid, model);
        let (sender, receiver) = mpsc::channel();
        let node_c = self.clone();
        let _t = thread::spawn(move || {
            let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            if model == "tiny".to_string() {
                let _stt = node_c.whisper_stt_tiny("/opt/thalamus/test.wav".to_string()).unwrap();
            }
            if model == "base".to_string() {
                let _stt = node_c.whisper_stt_base("/opt/thalamus/test.wav".to_string()).unwrap();
            }
            if model == "medium".to_string() {
                let _stt = node_c.whisper_stt_medium("/opt/thalamus/test.wav".to_string()).unwrap();
            }
            if model == "large".to_string() {
                let _stt = node_c.whisper_stt_large("/opt/thalamus/test.wav".to_string()).unwrap();
            } 
           
            let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            let time_elapsed = Some(end_timestamp - start_timestamp);

            match sender.send(time_elapsed) {
                Ok(()) => {}, // everything good
                Err(_) => {}, // we have been released, don't panic
            }
        });
        return receiver.recv_timeout(std::time::Duration::from_secs(100));
    }

    pub fn test_whisper_vwav(&self, model: String) -> Result<std::option::Option<i64>, std::sync::mpsc::RecvTimeoutError>{
        log::info!("{}: Running Whisper VWAV {} test...", self.pid, model);
        let (sender, receiver) = mpsc::channel();
        let node_c = self.clone();
        let _t = thread::spawn(move || {
            let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            if model == "tiny".to_string() {
                let _stt = node_c.whisper_vwav_tiny("/opt/thalamus/test.wav".to_string()).unwrap();
            }
            if model == "base".to_string() {
                let _stt = node_c.whisper_vwav_base("/opt/thalamus/test.wav".to_string()).unwrap();
            }
            if model == "medium".to_string() {
                let _stt = node_c.whisper_vwav_medium("/opt/thalamus/test.wav".to_string()).unwrap();
            }
            if model == "large".to_string() {
                let _stt = node_c.whisper_vwav_large("/opt/thalamus/test.wav".to_string()).unwrap();
            } 
           
            let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            let time_elapsed = Some(end_timestamp - start_timestamp);

            match sender.send(time_elapsed) {
                Ok(()) => {}, // everything good
                Err(_) => {}, // we have been released, don't panic
            }
        });
        return receiver.recv_timeout(std::time::Duration::from_secs(100));
    }
}

/// Struct for storing the jobs of each node
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThalamusNodeJob {
    pub oid: String,
    pub job_identifier: String,
    pub url: Option<String>,
    pub status: Option<String>,
    pub progress: Option<f64>,
    pub started_at: i64,
}
impl ThalamusNodeJob {
    pub fn new(job_identifier: String) -> ThalamusNodeJob {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        ThalamusNodeJob { 
            oid: oid,
            job_identifier: job_identifier,
            url: None,
            status: None,
            progress: None,
            started_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        }
    }
}

/// Struct for storing the stats of each node
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThalamusNodeStats {
    pub tts_score: Option<i64>,
    pub llama_7b: Option<i64>,
    pub llama_13b: Option<i64>,
    pub llama_30b: Option<i64>,
    pub llama_65b: Option<i64>,
    pub llama_score: Option<i64>,
    pub nst_score: Option<i64>,
    pub srgan_score: Option<i64>,
    pub whisper_stt_tiny: Option<i64>,
    pub whisper_stt_base: Option<i64>,
    pub whisper_stt_medium: Option<i64>,
    pub whisper_stt_large: Option<i64>,
    pub whisper_stt_score: Option<i64>,
    pub whisper_vwav_tiny: Option<i64>,
    pub whisper_vwav_base: Option<i64>,
    pub whisper_vwav_medium: Option<i64>,
    pub whisper_vwav_large: Option<i64>,
    pub whisper_vwav_score: Option<i64>,
}
impl ThalamusNodeStats {
    pub fn new() -> ThalamusNodeStats {
        ThalamusNodeStats { 
            whisper_stt_tiny: None,
            whisper_stt_base: None,
            whisper_stt_medium: None,
            whisper_stt_large: None,
            whisper_stt_score: None,
            llama_7b: None,
            llama_13b: None,
            llama_30b: None,
            llama_65b: None,
            llama_score: None,
            whisper_vwav_tiny: None,
            whisper_vwav_base: None,
            whisper_vwav_medium: None,
            whisper_vwav_large: None,
            whisper_vwav_score: None, 
            srgan_score: None,
            tts_score: None,
            nst_score: None
        }
    }

    pub fn calculate(node: ThalamusNode) -> ThalamusNodeStats {

        log::info!("Calculating stats for node {}.....", node.pid);
        
        // Test STT Tiny
        let mut whisper_stt_tiny: Option<i64> = None;
        let whisper_stt_tiny_test = node.test_whisper_stt("tiny".to_string());
        match whisper_stt_tiny_test {
            Ok(time_elapsed) => {
                whisper_stt_tiny = time_elapsed;
            },
            Err(e) => {
                log::error!("{}: Error running STT Tiny test: {:?}", node.pid, e);
            }
        }
        log::info!("{}: STT Tiny test complete in {:?} miliseconds", node.pid, whisper_stt_tiny);

        // Test STT Base
        let mut whisper_stt_base: Option<i64> = None;
        if whisper_stt_tiny.is_some(){
            let whisper_stt_base_test = node.test_whisper_stt("base".to_string());
            match whisper_stt_base_test {
                Ok(time_elapsed) => {
                    whisper_stt_base = time_elapsed;
                },
                Err(e) => {
                    log::error!("{}: Error running STT Base test: {:?}", node.pid, e);
                }
            }
            log::info!("{}: STT Base test complete in {:?} miliseconds", node.pid, whisper_stt_base);    
        }

        // Test STT Medium
        let mut whisper_stt_medium: Option<i64> = None;
        if whisper_stt_base.is_some() {
            let whisper_stt_medium_test = node.test_whisper_stt("medium".to_string());
            match whisper_stt_medium_test {
                Ok(time_elapsed) => {
                    whisper_stt_medium = time_elapsed;
                },
                Err(e) => {
                    log::error!("{}: Error running STT Medium test: {:?}", node.pid, e);
                }
            }
            log::info!("{}: STT Medium test complete in {:?} miliseconds", node.pid, whisper_stt_medium);    
        }

        // Test STT Large
        let mut whisper_stt_large: Option<i64> = None;
        if whisper_stt_medium.is_some() {
            let whisper_stt_large_test = node.test_whisper_stt("large".to_string());
            match whisper_stt_large_test {
                Ok(time_elapsed) => {
                    whisper_stt_large = time_elapsed;
                },
                Err(e) => {
                    log::error!("{}: Error running STT Large test: {:?}", node.pid, e);
                }
            }
            log::info!("{}: STT Large test complete in {:?} miliseconds", node.pid, whisper_stt_large);
        }

        // Calculate average STT score
        let mut stt_score = 0;
        match whisper_stt_tiny {
            Some(count) => {
                stt_score = count;
            },
            None => {}
        }
        match whisper_stt_base {
            Some(count) => {
                stt_score  = stt_score + count / 2;
            },
            None => {}
        }
        match whisper_stt_medium {
            Some(count) => {
                stt_score  = stt_score + count / 2;
            },
            None => {}
        }
        match whisper_stt_large {
            Some(count) => {
                stt_score  = stt_score + count / 2;
            },
            None => {}
        }

        let mut final_stt_score: Option<i64> = None;
        if stt_score > 0 {
            final_stt_score = Some(stt_score);
        }


        // Test VWAV Tiny
        let mut whisper_vwav_tiny: Option<i64> = None;
        let whisper_vwav_tiny_test = node.test_whisper_vwav("tiny".to_string());
        match whisper_vwav_tiny_test {
            Ok(time_elapsed) => {
                whisper_vwav_tiny = time_elapsed;
            },
            Err(e) => {
                log::error!("{}: Error running VWAV Tiny test: {:?}", node.pid, e);
            }
        }
        log::info!("{}: VWAV Tiny test complete in {:?} miliseconds", node.pid, whisper_vwav_tiny);

        // Test VWAV Base
        let mut whisper_vwav_base: Option<i64> = None;
        if whisper_vwav_tiny.is_some() {
            let whisper_vwav_base_test = node.test_whisper_vwav("base".to_string());
            match whisper_vwav_base_test {
                Ok(time_elapsed) => {
                    whisper_vwav_base = time_elapsed;
                },
                Err(e) => {
                    log::error!("{}: Error running VWAV Base test: {:?}", node.pid, e);
                }
            }
            log::info!("{}: VWAV Base test complete in {:?} miliseconds", node.pid, whisper_vwav_base);
        }

        // Test VWAV Medium
        let mut whisper_vwav_medium: Option<i64> = None;
        if whisper_vwav_base.is_some() {
            let whisper_vwav_medium_test = node.test_whisper_vwav("medium".to_string());
            match whisper_vwav_medium_test {
                Ok(time_elapsed) => {
                    whisper_vwav_medium = time_elapsed;
                },
                Err(e) => {
                    log::error!("{}: Error running VWAV Medium test: {:?}", node.pid, e);
                }
            }
            log::info!("{}: VWAV Medium test complete in {:?} miliseconds", node.pid, whisper_vwav_medium);
        }

        // Test VWAV Large
        let mut whisper_vwav_large: Option<i64> = None;
        if whisper_vwav_medium.is_some() {
            let whisper_vwav_large_test = node.test_whisper_vwav("large".to_string());
            match whisper_vwav_large_test {
                Ok(time_elapsed) => {
                    whisper_vwav_large = time_elapsed;
                },
                Err(e) => {
                    log::error!("{}: Error running VWAV Large test: {:?}", node.pid, e);
                }
            }
            log::info!("{}: VWAV Large test complete in {:?} miliseconds", node.pid, whisper_vwav_large);
        }

        // Calculate average VWAV score
        let mut vwav_score = 0;
        match whisper_vwav_tiny {
            Some(count) => {
                vwav_score = count;
            },
            None => {}
        }
        match whisper_vwav_base {
            Some(count) => {
                vwav_score  = vwav_score + count / 2;
            },
            None => {}
        }
        match whisper_vwav_medium {
            Some(count) => {
                vwav_score  = vwav_score + count / 2;
            },
            None => {}
        }
        match whisper_vwav_large {
            Some(count) => {
                vwav_score  = vwav_score + count / 2;
            },
            None => {}
        }

        let mut final_vwav_score: Option<i64> = None;
        if vwav_score > 0 {
            final_vwav_score = Some(vwav_score);
        }

        // Test LLAMA 7B
        let mut llama_7b: Option<i64> = None;
        let llama_7b_test = node.test_llama("7B".to_string());
        match llama_7b_test {
            Ok(time_elapsed) => {
                llama_7b = time_elapsed;
            },
            Err(e) => {
                log::error!("{}: Error running Llama 7B test: {:?}", node.pid, e);
            }
        }
        log::info!("{}: LLAMA 7B test complete in {:?} miliseconds", node.pid, llama_7b);

        // // Test LLAMA 13B
        // let mut llama_13b: Option<i64> = None;
        // if llama_7b.is_some(){
        //     let llama_13b_test = node.test_llama("13B".to_string());
        //     match llama_13b_test {
        //         Ok(time_elapsed) => {
        //             llama_13b = time_elapsed;
        //         },
        //         Err(e) => {
        //             log::error!("{}: Error running Llama 13B test: {:?}", node.pid, e);
        //         }
        //     }
        //     log::info!("{}: LLAMA 13B test complete in {:?} miliseconds", node.pid, llama_13b);    
        // }

        // // Test LLAMA 30B
        // let mut llama_30b: Option<i64> = None;
        // if llama_13b.is_some(){
        //     let llama_30b_test = node.test_llama("30B".to_string());
        //     match llama_30b_test {
        //         Ok(time_elapsed) => {
        //             llama_30b = time_elapsed;
        //         },
        //         Err(e) => {
        //             log::error!("{}: Error running Llama 30B test: {:?}", node.pid, e);
        //         }
        //     }
        //     log::info!("{}: LLAMA 30B test complete in {:?} miliseconds", node.pid, llama_30b);
        // }

        // // Test LLAMA 65B
        // let mut llama_65b: Option<i64> = None;
        // if llama_30b.is_some(){
        //     let llama_65b_test = node.test_llama("65B".to_string());
        //     match llama_65b_test {
        //         Ok(time_elapsed) => {
        //             llama_65b = time_elapsed;
        //         },
        //         Err(e) => {
        //             log::error!("{}: Error running Llama 65B test: {:?}", node.pid, e);
        //         }
        //     }
        //     log::info!("{}: LLAMA 65B test complete in {:?} miliseconds", node.pid, llama_65b);
        // }

        // Calculate average llama score
        let mut llama_score = 0;
        match llama_7b {
            Some(llama_7bx) => {
                llama_score  = llama_7bx;
            },
            None => {}
        }
        // match llama_13b {
        //     Some(llama_13bx) => {
        //         llama_score  = llama_score + llama_13bx / 2;
        //     },
        //     None => {}
        // }
        // match llama_30b {
        //     Some(llama_30bx) => {
        //         llama_score  = llama_score + llama_30bx / 2;
        //     },
        //     None => {}
        // }
        // match llama_65b {
        //     Some(llama_65bx) => {
        //         llama_score  = llama_score + llama_65bx / 2;
        //     },
        //     None => {}
        // }

        let mut final_llama_score: Option<i64> = None;
        if llama_score > 0 {
            final_llama_score = Some(llama_score);
        }

        // Test SRGAN
        let mut srgan: Option<i64> = None;
        let srgan_test = node.test_srgan();
        match srgan_test {
            Ok(time_elapsed) => {
                srgan = time_elapsed;
            },
            Err(e) => {
                log::error!("{}: Error running SRGAN test: {:?}", node.pid, e);
            }
        }
        log::info!("{}: SRGAN test complete in {:?} miliseconds", node.pid, srgan);

        // Test TTS
        let mut tts: Option<i64> = None;
        let tts_test = node.test_tts();
        match tts_test {
            Ok(time_elapsed) => {
                tts = time_elapsed;
            },
            Err(e) => {
                log::error!("{}: Error running TTS test: {:?}", node.pid, e);
            }
        }
        log::info!("{}: TTS test complete in {:?} miliseconds", node.pid, tts);

        

        // Return stats
        return ThalamusNodeStats { 
            whisper_stt_tiny: whisper_stt_tiny,
            whisper_stt_base: whisper_stt_base,
            whisper_stt_medium: whisper_stt_medium,
            whisper_stt_large: whisper_stt_large,
            whisper_stt_score: final_stt_score,
            llama_7b: llama_7b,
            llama_13b: None,
            llama_30b: None,
            llama_65b: None,
            llama_score: final_llama_score,
            whisper_vwav_tiny: whisper_vwav_tiny,
            whisper_vwav_base: whisper_vwav_base,
            whisper_vwav_medium: whisper_vwav_medium,
            whisper_vwav_large: whisper_vwav_large,
            whisper_vwav_score: final_vwav_score, 
            srgan_score: srgan,
            tts_score: tts,
            nst_score: None
        };
    }
}

/// Struct for storing the stats of each node
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThalamusNodeCapability {
    pub tag: String,
}

/// Auxilary Struct for API Version replies
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionReply {
    pub version: String,
    pub pid: String,
}

/// Auxilary Struct for API STT replies
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct STTReply {
    pub text: String,
    pub time: f64,
    pub response_type: Option<String>,
}