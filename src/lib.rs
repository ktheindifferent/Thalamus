// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use local_ip_address::list_afinet_netifas;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};


use tokio::task;
use std::thread;
use std::sync::mpsc;

extern crate rouille;

pub mod thalamus;
pub mod p2p;






/// Struct for storing all nodes connected to the client
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThalamusClient {
    pub nodes: Arc<Mutex<Vec<ThalamusNode>>>,
}
impl ThalamusClient {
    pub fn new() -> ThalamusClient {
        let x: Vec<ThalamusNode> = Vec::new();
        ThalamusClient { 
            nodes: Arc::new(Mutex::new(x)),
        }
    }

    pub fn ipv4_discovery(&mut self){

        let network_interfaces = list_afinet_netifas().unwrap();

        let nodex = Arc::clone(&self.nodes);

        for (name, ip) in network_interfaces.iter() {
            if !ip.is_loopback() && !format!("{}", ip.clone()).contains(":"){
                log::warn!("{}:\t{:?}", name, ip);
                let ips = crate::thalamus::tools::netscan::scan_bulk(format!("{}", ip).as_str(), "8050", "/24").unwrap();
                log::warn!("Found {} ips", ips.len());
            
                // Check matching ips for thalamus version info
                for ipx in ips{
                    let version = fetch_version(ipx.as_str());
                    match version {
                        Ok(v) => {
                            let mut nodes = nodex.lock().unwrap();
                            let existing_index = nodes.clone().iter().position(|r| r.pid == v.pid.to_string());
                            match existing_index {
                                Some(_index) => {
                                },
                                None => {
                                    nodes.push(ThalamusNode::new(v.pid.to_string(), v.version.to_string(), ipx, 8050));
                                }
                            }
                            std::mem::drop(nodes);
                        },
                        Err(e) => {
                            log::error!("fetch_thalamus_version_error: {}", e);
                        }
                    }
                }
                
            }
           
        }


        
    }

    pub async fn mdns_discovery(&mut self, discovery: simple_mdns::async_discovery::ServiceDiscovery) -> Result<simple_mdns::async_discovery::ServiceDiscovery, std::io::Error> {
        let nodex = Arc::clone(&self.nodes);

        let services = discovery.get_known_services().await;
        if services.len() > 0 {
            for xy in services{
                log::info!("vhhjv: {:?}", xy);
                // Register using ip address
                for ipfx in xy.ip_addresses.clone(){
                    let ipx = ipfx.to_string();
                    let port = xy.ports[0];
                    if !ipx.to_string().contains(".0.1"){
                        let version = async_fetch_version(format!("{}:{}", ipx, port).as_str()).await;
                        match version {
                            Ok(v) => {
                                let nodess = nodex.lock().unwrap();
                                let nodes = nodess.clone();
                                std::mem::drop(nodess);
                                let existing_index = nodes.clone().iter().position(|r| r.pid == v.pid.to_string());
                                match existing_index {
                                    Some(index) => {
                                        let mut nodes = nodex.lock().unwrap();
                                        nodes[index].is_online = true;
                                        nodes[index].last_ping = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
                                        
                                        log::info!("NODE_ONLINE: {:?}", nodes[index].clone());
                                        std::mem::drop(nodes);
                                        self.save();
                                    },
                                    None => {
                                        let thalamus_node = ThalamusNode::new(v.pid.to_string(), v.version.to_string(), format!("{}:{}", ipx, port), 8050);
                                        let mut nodes = nodex.lock().unwrap();
                                        log::info!("NEW_NODE: {:?}", thalamus_node.clone());
                                        nodes.push(thalamus_node);
                                        std::mem::drop(nodes);
                                        self.save();
                                    }
                                }
                                
                            },
                            Err(e) => {
                                log::error!("fetch_thalamus_version_error: {}", e);
                                let nodess = nodex.lock().unwrap();
                                let nodes = nodess.clone();
                                std::mem::drop(nodess);
                                let existing_index = nodes.clone().iter().position(|r| r.ip_address == format!("{}:{}", ipx, port).as_str());
                                match existing_index {
                                    Some(index) => {
                                        let mut nodes = nodex.lock().unwrap();
                                        nodes[index].is_online = false;
                                        log::info!("NODE_OFFLINE: {:?}", nodes[index].clone());
                                        std::mem::drop(nodes);
                                        self.save();
                                    },
                                    None => {
                                        // std::mem::drop(nodes);
                                    }
                                }
                            }
                        }
                    }
                }

                // TODO: flag missing nodes as offline
                let mut nodey = nodex.lock().unwrap();
                for node in nodey.iter_mut() {
                    let existing_index = xy.ip_addresses.clone().iter().position(|r| node.ip_address.contains(format!("{}", r).as_str()));
                    match existing_index {
                        Some(_) => {

                        },
                        None => {
                            node.is_online = false;
                            
                            self.save();
                        }
                    }
                }
                std::mem::drop(nodey);

            }
        }

        Ok(discovery)
    }

    // pub fn start_mdns_responder(&mut self){

    // }
    
    pub async fn nodex_discovery(&mut self){
        let nodell = self.nodes.lock().unwrap();
        let nodess = nodell.clone();
        std::mem::drop(nodell);
        for node in nodess{
            let nodexs_wrap = node.nodex();
            match nodexs_wrap {
                Ok(nodexs) => {
                    for nodex in nodexs{
                        let nodess = self.nodes.lock().unwrap();
                        let nodes = nodess.clone();
                        std::mem::drop(nodess);
                        let existing_index = nodes.clone().iter().position(|r| r.pid == nodex.pid.to_string());
                        match existing_index {
                            Some(_) => {

                            },
                            None => {
                                let mut nodes = self.nodes.lock().unwrap();
                                nodes.push(nodex);
                                std::mem::drop(nodes);
                                self.save();
                            }
                        }
                    }
                },
                Err(e) => {
                    log::error!("nodex_discovery_error: {}", e);
                }
            }

        }
    }

    pub fn save(&self){
        std::fs::File::create("/opt/thalamusc/clients.json").expect("create failed");
        let j = serde_json::to_string(&self).unwrap();
        std::fs::write("/opt/thalamusc/clients.json", j).expect("Unable to write file");


        let nodex = self.nodes.lock().unwrap();
        let nodes = nodex.clone();
        std::mem::drop(nodex);
        if nodes.len() > 0 {
            std::fs::File::create("/opt/thalamusc/clients.bak.json").expect("create failed");
            let j = serde_json::to_string(&self).unwrap();
            std::fs::write("/opt/thalamusc/clients.bak.json", j).expect("Unable to write file");
        }
    }

    pub fn load(retries: i64) -> Result<ThalamusClient, Box<dyn Error>>{

        if !std::path::Path::new("/opt/thalamusc/clients.json").exists(){
            let new_c = ThalamusClient::new();
            new_c.save();
            return Ok(new_c);
        }

        let save_file = std::fs::read_to_string("/opt/thalamusc/clients.json");
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
                            std::fs::copy("/opt/thalamusc/clients.bak.json", "/opt/thalamusc/clients.json")?;
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
                    std::fs::copy("/opt/thalamusc/clients.bak.json", "/opt/thalamusc/clients.json")?;
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

    //     let mut fastest_whisper_stt_score = 9999999;
    //     let mut fastest_whisper_vwav_score = 9999999;
    //     let mut fastest_srgan_score = 9999999;
    //     let mut fastest_llama_score = 9999999;
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



pub fn fetch_version(host: &str) -> Result<VersionReply, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    return Ok(client.get(format!("http://{}/api/thalamus/version", host)).send()?.json()?);
}

pub async fn async_fetch_version(host: &str) -> Result<VersionReply, Box<dyn Error>> {
    let client = reqwest::Client::builder().build()?;
    return Ok(client.get(format!("http://{}/api/thalamus/version", host)).send().await?.json().await?);
}





/// Struct for storing node information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThalamusNode {
    pub pid: String,
    pub ip_address: String, // unique
    pub version: String,
    pub port: u16,
    pub jobs: Vec<ThalamusNodeJob>,
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
            port: port,
            last_ping: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            stats: ThalamusNodeStats::new(),
            is_online: true,
        };
        let stats = ThalamusNodeStats::calculate(node.clone());
        node.stats = stats;
        return node;
    }

    pub fn whisper_stt_tiny(&self, tmp_file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "tiny").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}/api/services/whisper", self.ip_address.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn whisper_stt_base(&self, tmp_file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "basic").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}/api/services/whisper", self.ip_address.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn whisper_stt_medium(&self, tmp_file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "medium").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}/api/services/whisper", self.ip_address.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn whisper_stt_large(&self, tmp_file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "large").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}/api/services/whisper", self.ip_address.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn whisper_vwav_tiny(&self, tmp_file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "tiny").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}/api/services/whisper/vwav", self.ip_address.clone()))
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn whisper_vwav_base(&self, tmp_file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "base").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}/api/services/whisper/vwav", self.ip_address.clone()))
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn whisper_vwav_medium(&self, tmp_file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "medium").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}/api/services/whisper/vwav", self.ip_address.clone()))
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn whisper_vwav_large(&self, tmp_file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "large").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}/api/services/whisper/vwav", self.ip_address.clone()))
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn srgan(&self, tmp_file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{

        let parts: Vec<&str> = tmp_file_path.split('.').collect();

        let extension = parts[parts.len() - 1];

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;

        let new_file_name = format!("{}.{}", timestamp, extension);

        let form = reqwest::blocking::multipart::Form::new().text("filename", new_file_name).file("input_file", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}/api/services/image/srgan", self.ip_address.clone()))
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn llama(&self, prompt: String, model: String) -> Result<String, Box<dyn Error>>{
        let params = [("model", model.as_str()), ("prompt", prompt.as_str())];

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}/api/services/llama", self.ip_address.clone()))
        .form(&params)
        .send()?.text()?;

        return Ok(bytes.to_string());
    }

    pub fn tts(&self, prompt: String) -> Result<Vec<u8>, Box<dyn Error>>{
        let params = [("prompt", prompt.as_str())];

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}/api/services/tts", self.ip_address.clone()))
        .form(&params)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn nodex(&self) -> Result<Vec<ThalamusNode>, Box<dyn Error>>{
        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let mut url = format!("http://{}/api/nodex", self.ip_address.clone());
        if !url.contains(":") {
            url = format!("{}:{}", url, self.port.clone());
        }

        return Ok(client.get(url)
        .send()?.json()?);
    }

    pub fn test_srgan(&self) -> Result<std::option::Option<i64>, std::sync::mpsc::RecvTimeoutError>{
        log::info!("{}: Running SRGAN test...", self.pid);
        let (sender, receiver) = mpsc::channel();
        let node_c = self.clone();
        let _t = thread::spawn(move || {
            let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
            let _srgan = node_c.srgan("/opt/thalamusc/test.jpg".to_string()).unwrap();
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
                let _stt = node_c.whisper_stt_tiny("/opt/thalamusc/test.wav".to_string()).unwrap();
            }
            if model == "base".to_string() {
                let _stt = node_c.whisper_stt_base("/opt/thalamusc/test.wav".to_string()).unwrap();
            }
            if model == "medium".to_string() {
                let _stt = node_c.whisper_stt_medium("/opt/thalamusc/test.wav".to_string()).unwrap();
            }
            if model == "large".to_string() {
                let _stt = node_c.whisper_stt_large("/opt/thalamusc/test.wav".to_string()).unwrap();
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
                let _stt = node_c.whisper_vwav_tiny("/opt/thalamusc/test.wav".to_string()).unwrap();
            }
            if model == "base".to_string() {
                let _stt = node_c.whisper_vwav_base("/opt/thalamusc/test.wav".to_string()).unwrap();
            }
            if model == "medium".to_string() {
                let _stt = node_c.whisper_vwav_medium("/opt/thalamusc/test.wav".to_string()).unwrap();
            }
            if model == "large".to_string() {
                let _stt = node_c.whisper_vwav_large("/opt/thalamusc/test.wav".to_string()).unwrap();
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
    pub url: String,
    pub started_at: i64,
}
impl ThalamusNodeJob {
    pub fn new() -> ThalamusNodeJob {
        let oid: String = thread_rng().sample_iter(&Alphanumeric).take(15).map(char::from).collect();
        ThalamusNodeJob { 
            oid: oid,
            url: String::new(),
            started_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        }
    }
}

/// Struct for storing the stats of each node
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThalamusNodeStats {
    pub apple_tts: Option<i64>,
    pub bark_tts: Option<i64>,
    pub deepspeech_tts: Option<i64>,
    pub espeak_tts: Option<i64>,
    pub watson_tts: Option<i64>,
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
            espeak_tts: None,
            apple_tts: None,
            bark_tts: None,
            watson_tts: None,
            deepspeech_tts: None,
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



        // let whisper_vwav_score = (whisper_vwav_tiny + whisper_vwav_base + whisper_vwav_medium + whisper_vwav_large) / 4;

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

        // Test LLAMA 13B
        let mut llama_13b: Option<i64> = None;
        if llama_7b.is_some(){
            let llama_13b_test = node.test_llama("13B".to_string());
            match llama_13b_test {
                Ok(time_elapsed) => {
                    llama_13b = time_elapsed;
                },
                Err(e) => {
                    log::error!("{}: Error running Llama 13B test: {:?}", node.pid, e);
                }
            }
            log::info!("{}: LLAMA 13B test complete in {:?} miliseconds", node.pid, llama_13b);    
        }

        // Test LLAMA 30B
        let mut llama_30b: Option<i64> = None;
        if llama_13b.is_some(){
            let llama_30b_test = node.test_llama("30B".to_string());
            match llama_30b_test {
                Ok(time_elapsed) => {
                    llama_30b = time_elapsed;
                },
                Err(e) => {
                    log::error!("{}: Error running Llama 30B test: {:?}", node.pid, e);
                }
            }
            log::info!("{}: LLAMA 30B test complete in {:?} miliseconds", node.pid, llama_30b);
        }

        // Test LLAMA 65B
        let mut llama_65b: Option<i64> = None;
        if llama_30b.is_some(){
            let llama_65b_test = node.test_llama("65B".to_string());
            match llama_65b_test {
                Ok(time_elapsed) => {
                    llama_65b = time_elapsed;
                },
                Err(e) => {
                    log::error!("{}: Error running Llama 65B test: {:?}", node.pid, e);
                }
            }
            log::info!("{}: LLAMA 65B test complete in {:?} miliseconds", node.pid, llama_65b);
        }

        // Calculate average llama score
        let mut llama_score = 0;
        match llama_7b {
            Some(llama_7bx) => {
                llama_score  = llama_7bx;
            },
            None => {}
        }
        match llama_13b {
            Some(llama_13bx) => {
                llama_score  = llama_score + llama_13bx / 2;
            },
            None => {}
        }
        match llama_30b {
            Some(llama_30bx) => {
                llama_score  = llama_score + llama_30bx / 2;
            },
            None => {}
        }
        match llama_65b {
            Some(llama_65bx) => {
                llama_score  = llama_score + llama_65bx / 2;
            },
            None => {}
        }

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

        // Return stats
        return ThalamusNodeStats { 
            whisper_stt_tiny: whisper_stt_tiny,
            whisper_stt_base: whisper_stt_base,
            whisper_stt_medium: whisper_stt_medium,
            whisper_stt_large: whisper_stt_large,
            whisper_stt_score: final_stt_score,
            llama_7b: llama_7b,
            llama_13b: llama_13b,
            llama_30b: llama_30b,
            llama_65b: llama_65b,
            llama_score: final_llama_score,
            whisper_vwav_tiny: whisper_vwav_tiny,
            whisper_vwav_base: whisper_vwav_base,
            whisper_vwav_medium: whisper_vwav_medium,
            whisper_vwav_large: whisper_vwav_large,
            whisper_vwav_score: final_vwav_score, 
            srgan_score: srgan,
            espeak_tts: None,
            apple_tts: None,
            bark_tts: None,
            watson_tts: None,
            deepspeech_tts: None,
            tts_score: None,
            nst_score: None
        };
    }
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