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
use simple_dns::{Name, CLASS, ResourceRecord, rdata::{RData, A, SRV}};
use simple_mdns::async_discovery::SimpleMdnsResponder;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{net::IpAddr};
use tokio::task::yield_now;
use tokio::task;

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
                // TODO: Register 
                for ipfx in xy.ip_addresses{
                    let ipx = ipfx.to_string();
                    let port = xy.ports[0];
                    if !ipx.to_string().contains(".0.1"){
                        let version = async_fetch_version(format!("{}:{}", ipx, port).as_str()).await;
                        match version {
                            Ok(v) => {
                                let mut nodes = nodex.lock().unwrap();
                                let existing_index = nodes.clone().iter().position(|r| r.pid == v.pid.to_string());
                                match existing_index {
                                    Some(index) => {
                                        nodes[index].is_online = true;
                                        nodes[index].last_ping = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
                                        std::mem::drop(nodes);
                                        self.save();
                                    },
                                    None => {
                                        nodes.push(ThalamusNode::new(v.pid.to_string(), v.version.to_string(), format!("{}:{}", ipx, port), 8050));
                                        std::mem::drop(nodes);
                                        self.save();
                                    }
                                }
                                
                            },
                            Err(e) => {
                                log::error!("fetch_thalamus_version_error: {}", e);
                                let mut nodes = nodex.lock().unwrap();
                                let existing_index = nodes.clone().iter().position(|r| r.ip_address == format!("{}:{}", ipx, port).as_str());
                                match existing_index {
                                    Some(index) => {
                                        nodes[index].is_online = false;
                                        std::mem::drop(nodes);
                                        self.save();
                                    },
                                    None => {
                                        std::mem::drop(nodes);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(discovery)
    }

    pub async fn start_mdns_responder(&mut self){
        let network_interfaces = list_afinet_netifas().unwrap();
        task::spawn(async move{
            
    
            let mut responder = SimpleMdnsResponder::new(10);
            let srv_name = Name::new_unchecked("_thalamus._tcp.local");
        
            for (_name, ip) in network_interfaces.iter() {
                if !ip.is_loopback() && !format!("{}", ip.clone()).contains(":") && !format!("{}", ip.clone()).contains(".0.1"){
                    match *ip {
                        IpAddr::V4(ipv4) => { 
                            responder.add_resource(ResourceRecord::new(
                                srv_name.clone(),
                                CLASS::IN,
                                10,
                                RData::A(A { address: ipv4.into() }),
                            )).await;
                         },
                        IpAddr::V6(_ipv6) => { /* handle IPv6 */ }
                    }
    
                    
                }
            }
        
            responder.add_resource(ResourceRecord::new(
                srv_name.clone(),
                CLASS::IN,
                10,
                RData::SRV(SRV {
                    port: 8050,
                    priority: 0,
                    weight: 0,
                    target: srv_name
                })
            )).await;
    
            yield_now().await;
            
        });
    }
    
    pub async fn nodex_discovery(&mut self){
        let nodell = self.nodes.lock().unwrap();
        let nodess = nodell.clone();
        std::mem::drop(nodell);
        for node in nodess{
            let nodexs_wrap = node.nodex();
            match nodexs_wrap {
                Ok(nodexs) => {
                    for nodex in nodexs{
                        let mut nodes = self.nodes.lock().unwrap();
                        let existing_index = nodes.clone().iter().position(|r| r.pid == nodex.pid.to_string());
                        match existing_index {
                            Some(index) => {
                                nodes[index].last_ping = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
                                std::mem::drop(nodes);
                                self.save();
                            },
                            None => {
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
    }

    pub fn load() -> Result<ThalamusClient, Box<dyn Error>>{
        let save_file = std::fs::read_to_string("/opt/thalamusc/clients.json");
        match save_file {
            Ok(save_data) => {
                let v: Result<ThalamusClient, _> = serde_json::from_str(&save_data);
                match v {
                    Ok(v2) => {
                        return Ok(v2);
                    },
                    Err(e) => {
                        log::error!("{}", format!("Unable to read save file: {}", e));
                        let new_c = ThalamusClient::new();
                        new_c.save();
                        return Ok(new_c);
                    }
                }
                
            },
            Err(e) => {
                log::error!("{}", format!("Unable to read save file: {}", e));
                let new_c = ThalamusClient::new();
                new_c.save();
                return Ok(new_c);
            }
        }
    }

    pub fn select_optimal_node(&self, node_type: String) -> Result<ThalamusNode, Box<dyn Error + '_>> {
        let nodex = self.nodes.lock()?;
        let nodes = nodex.clone();
        std::mem::drop(nodex);

        let mut fastest_stt_score = 9999999;
        let mut fastest_vwav_score = 9999999;
        let mut fastest_srgan_score = 9999999;
        let mut fastest_llama_score = 9999999;
        let mut selected_node = nodes[0].clone();
        for node in nodes {
            let stats = node.stats.clone();
            if stats.stt_score < fastest_stt_score && node_type.contains("stt") {
                fastest_stt_score = stats.stt_score;
                selected_node = node.clone();
            }
            if stats.vwav_score < fastest_vwav_score && node_type.contains("vwav") {
                fastest_vwav_score = stats.vwav_score;
                selected_node = node.clone();
            }
            if stats.srgan < fastest_srgan_score && node_type.contains("srgan") {
                fastest_srgan_score = stats.srgan;
                selected_node = node.clone();
            }
            if stats.llama_score < fastest_llama_score && node_type.contains("llama") {
                fastest_llama_score = stats.llama_score;
                selected_node = node.clone();
            }
        }
        
        return Ok(selected_node);
    }
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

    pub fn stt_tiny(&self, tmp_file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "tiny").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}/api/services/whisper", self.ip_address.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn stt_base(&self, tmp_file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "basic").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}/api/services/whisper", self.ip_address.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn stt_medium(&self, tmp_file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "medium").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}/api/services/whisper", self.ip_address.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn stt_large(&self, tmp_file_path: String) -> Result<STTReply, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "large").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}/api/services/whisper", self.ip_address.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn vwav_tiny(&self, tmp_file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "tiny").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}/api/services/whisper/vwav", self.ip_address.clone()))
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn vwav_base(&self, tmp_file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "base").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}/api/services/whisper/vwav", self.ip_address.clone()))
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn vwav_medium(&self, tmp_file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "medium").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}/api/services/whisper/vwav", self.ip_address.clone()))
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn vwav_large(&self, tmp_file_path: String) -> Result<Vec<u8>, Box<dyn Error>>{
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
    pub stt_tiny: i64,
    pub stt_base: i64,
    pub stt_medium: i64,
    pub stt_large: i64,
    pub stt_score: i64,
    pub llama_tiny: i64,
    pub llama_basic: i64,
    pub llama_medium: i64,
    pub llama_large: i64,
    pub llama_score: i64,
    pub vwav_tiny: i64,
    pub vwav_base: i64,
    pub vwav_medium: i64,
    pub vwav_large: i64,
    pub vwav_score: i64,
    pub srgan: i64,
    pub espeak_tts: i64,
    pub apple_tts: i64,
    pub google_tts: i64,
    pub watson_tts: i64,
    pub deepspeech_tts: i64,
    pub tts_score: i64,
    pub nst_score: i64,
}
impl ThalamusNodeStats {
    pub fn new() -> ThalamusNodeStats {
        ThalamusNodeStats { 
            stt_tiny: 0,
            stt_base: 0,
            stt_medium: 0,
            stt_large: 0,
            stt_score: 0,
            llama_tiny: 0,
            llama_basic: 0,
            llama_medium: 0,
            llama_large: 0,
            llama_score: 0,
            vwav_tiny: 0,
            vwav_base: 0,
            vwav_medium: 0,
            vwav_large: 0,
            vwav_score: 0, 
            srgan: 0,
            espeak_tts: 0,
            apple_tts: 0,
            google_tts: 0,
            watson_tts: 0,
            deepspeech_tts: 0,
            tts_score: 0,
            nst_score: 0
        }
    }

    pub fn calculate(node: ThalamusNode) -> ThalamusNodeStats {

        log::info!("Calculating stats for node {}.....", node.pid);

        // Test STT Tiny
        log::info!("{}: Running STT Tiny test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _stt = node.stt_tiny("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let tiny_stt = end_timestamp - start_timestamp;
        log::info!("{}: STT Tiny test complete in {} miliseconds", node.pid, tiny_stt);
        
        // Test STT Base
        log::info!("{}: Running STT Base test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _stt = node.stt_base("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let basic_stt = end_timestamp - start_timestamp;
        log::info!("{}: STT Base test complete in {} miliseconds", node.pid, basic_stt);
        
        // Test STT Medium
        log::info!("{}: Running STT Medium test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _stt = node.stt_medium("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let medium_stt = end_timestamp - start_timestamp;
        log::info!("{}: STT Medium test complete in {} miliseconds", node.pid, medium_stt);

        // Test STT Large
        log::info!("{}: Running STT Large test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _stt = node.stt_large("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let large_stt = end_timestamp - start_timestamp;
        log::info!("{}: STT Large test complete in {} miliseconds", node.pid, large_stt);
        
        // Calculate average STT score
        let stt_score = (tiny_stt + basic_stt + medium_stt + large_stt) / 4;

        // Test VWAV Tiny
        log::info!("{}: Running VWAV Tiny test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _stt = node.vwav_tiny("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let vwav_tiny = end_timestamp - start_timestamp;
        log::info!("{}: VWAV Tiny test complete in {} miliseconds", node.pid, vwav_tiny);
        
        // Test VWAV Base
        log::info!("{}: Running VWAV Base test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _stt = node.vwav_base("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let vwav_base = end_timestamp - start_timestamp;
        log::info!("{}: VWAV Base test complete in {} miliseconds", node.pid, vwav_base);
        
        // Test VWAV Medium
        log::info!("{}: Running VWAV Medium test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _stt = node.vwav_medium("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let vwav_medium = end_timestamp - start_timestamp;
        log::info!("{}: VWAV Medium test complete in {} miliseconds", node.pid, vwav_medium);

        // Test VWAV Large
        log::info!("{}: Running VWAV Large test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _stt = node.vwav_large("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let vwav_large = end_timestamp - start_timestamp;
        log::info!("{}: VWAV Large test complete in {} miliseconds", node.pid, vwav_large);

        // Calculate average VWAV score
        let vwav_score = (vwav_tiny + vwav_base + vwav_medium + vwav_large) / 4;

        // Test LLAMA 7B
        log::info!("{}: Running LLAMA 7B test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _llama = node.llama("Tell me about Abraham Lincoln.".to_string(), "7B".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let llama_tiny = end_timestamp - start_timestamp;
        log::info!("{}: LLAMA 7B test complete in {} miliseconds", node.pid, llama_tiny);

        // Test LLAMA 13B
        log::info!("{}: Running LLAMA 13B test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _llama = node.llama("Tell me about Abraham Lincoln.".to_string(), "13B".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let llama_basic = end_timestamp - start_timestamp;
        log::info!("{}: LLAMA 13B test complete in {} miliseconds", node.pid, llama_tiny);

        // Test LLAMA 30B
        log::info!("{}: Running LLAMA 30B test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _llama = node.llama("Tell me about Abraham Lincoln.".to_string(), "30B".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let llama_medium = end_timestamp - start_timestamp;
        log::info!("{}: LLAMA 30B test complete in {} miliseconds", node.pid, llama_tiny);

        // Test LLAMA 65B
        log::info!("{}: Running LLAMA 65B test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _llama = node.llama("Tell me about Abraham Lincoln.".to_string(), "65B".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let llama_large = end_timestamp - start_timestamp;
        log::info!("{}: LLAMA 65B test complete in {} miliseconds", node.pid, llama_tiny);

        // Calculate average llama score
        let llama_score = (llama_tiny + llama_basic + llama_medium + llama_large) / 4;

        // Test SRGAN
        log::info!("{}: Running SRGAN test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let _stt = node.srgan("/opt/thalamusc/test.jpg".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let srgan = end_timestamp - start_timestamp;
        log::warn!("{}: SRGAN test complete in {} miliseconds", node.pid, srgan);

        // Return stats
        return ThalamusNodeStats { 
            stt_tiny: tiny_stt,
            stt_base: basic_stt,
            stt_medium: medium_stt,
            stt_large: large_stt,
            stt_score: stt_score,
            llama_tiny: llama_tiny,
            llama_basic: llama_basic,
            llama_medium: llama_medium,
            llama_large: llama_large,
            llama_score: llama_score,
            vwav_tiny: vwav_tiny,
            vwav_base: vwav_base,
            vwav_medium: vwav_medium,
            vwav_large: vwav_large,
            vwav_score: vwav_score, 
            srgan: srgan,
            espeak_tts: 0,
            apple_tts: 0,
            google_tts: 0,
            watson_tts: 0,
            deepspeech_tts: 0,
            tts_score: 0,
            nst_score: 0
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