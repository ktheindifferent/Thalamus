use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use local_ip_address::local_ip;
use local_ip_address::list_afinet_netifas;

use rouille::Server;
use rouille::Response;
use simple_logger::SimpleLogger;
use std::path::Path;

extern crate rouille;

pub mod thalamus;

// store application version as a const
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

use error_chain::error_chain;
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub fn init(){
    // cls
    clearscreen::clear().unwrap();




    // Print Application Art and Version Information
    println!("████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ ");
    println!("   ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      ");
    println!("   ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ ");
    println!("   ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ ");
    println!("   ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████ ");
    println!("Copyright 2021-2023 The Open Sam Foundation (OSF)");
    match VERSION {
        Some(v) => println!("Version: {}", v),
        None => println!("Version: Unknown"),
    };


    if Path::new("/opt/thalamus/").exists() {
        let touch_status = crate::thalamus::tools::touch("/opt/thalamus/output.log".to_string());
        if touch_status.is_ok() {
            SimpleLogger::new().with_colors(true).with_level(log::LevelFilter::Warn).with_timestamps(true).with_output_file("/opt/thalamus/output.log".to_string()).init().unwrap();
        } else {
            SimpleLogger::new().with_colors(true).with_level(log::LevelFilter::Warn).with_timestamps(true).init().unwrap();
        }
    } else {
        simple_logger::SimpleLogger::new().with_colors(true).with_level(log::LevelFilter::Warn).with_timestamps(true).init().unwrap();
    }


    
    sudo::with_env(&["LIBTORCH", "LD_LIBRARY_PATH", "PG_DBNAME", "PG_USER", "PG_PASS", "PG_ADDRESS"]).unwrap();
    


    match std::env::current_exe() {
        Ok(exe_path) => {
            let current_exe_path = format!("{}", exe_path.display());

            if current_exe_path.as_str() == "/opt/thalamus/bin/thalamus"{
                let server = Server::new("0.0.0.0:8050", |request| {
                    match crate::thalamus::http::handle(request){
                        Ok(request) => {
                            log::info!("{:?}", request);
                            return request;
                        },
                        Err(err) => {
                            log::error!("HTTP_ERROR: {}", err);
                            return Response::empty_404();
                        }
                    }
                }).unwrap().pool_size(6);
            
                loop {
                    server.poll();
                }
            } else {
                match crate::thalamus::setup::install(){
                    Ok(_) => log::warn!("Installed thalamus"),
                    Err(e) => log::error!("Error installing thalamus: {}", e),
                };
            }
        },
        Err(e) => log::error!("failed to get current exe path: {e}"),
    };

}


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

  
    // TODO: Automatically discover thalamus nodes
    pub fn discover(&mut self){

        let network_interfaces = list_afinet_netifas().unwrap();

        for (name, ip) in network_interfaces.iter() {
            if !ip.is_loopback() && !format!("{}", ip.clone()).contains(":"){
                log::warn!("{}:\t{:?}", name, ip);
                let ips = crate::thalamus::tools::netscan::scan_bulk(format!("{}", ip).as_str(), "8050", "/24").unwrap();
                log::warn!("Found {} ips", ips.len());
            
                // TODO: Check matching ips for thalamus version info

                for ipx in ips{
                    let version = fetch_version(ipx.as_str());
                    match version {
                        Ok(v) => {
                            let existing_index = self.nodes.iter().position(|r| r.pid == v.pid.to_string());
                            match existing_index {
                                Some(index) => {
                                },
                                None => {
                                    self.nodes.push(ThalamusNode::new(v.pid.to_string(), v.version.to_string(), ipx, 8050));
                                }
                            }
                        },
                        Err(e) => {
                            log::error!("fetch_thalamus_version_error: {}", e);
                        }
                    }
                }
                
            }
           
        }


        
    }

    // TODO: Save client state to disk
    pub fn save(&self){

    }

    // TODO: Load client state from disk
    pub fn load() -> Result<ThalamusClient>{
        Ok(ThalamusClient::new())
    }

    pub fn select_optimal_node(&self, node_type: String) -> Result<ThalamusNode> {
        let nodes = self.nodes.clone();

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



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionReply {
    pub version: String,
    pub pid: String,
}
pub fn fetch_version(host: &str) -> Result<VersionReply> {
    let client = reqwest::blocking::Client::builder().build()?;
    return Ok(client.get(format!("http://{}/api/thalamus/version", host)).send()?.json()?);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct STTReply {
    pub text: String,
    pub time: f64,
    pub response_type: Option<String>,
}




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThalamusNode {
    pub pid: String,
    pub ip_address: String, // unique
    pub version: String,
    pub port: u16,
    pub jobs: Vec<ThalamusNodeJob>,
    pub last_ping: i64,
    pub stats: ThalamusNodeStats,
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
        };
        let stats = ThalamusNodeStats::calculate(node.clone());
        node.stats = stats;
        return node;
    }

    pub fn stt_tiny(&self, tmp_file_path: String) -> Result<STTReply>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "tiny").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}/api/services/whisper", self.ip_address.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn stt_basic(&self, tmp_file_path: String) -> Result<STTReply>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "basic").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}/api/services/whisper", self.ip_address.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn stt_medium(&self, tmp_file_path: String) -> Result<STTReply>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "medium").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}/api/services/whisper", self.ip_address.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn stt_large(&self, tmp_file_path: String) -> Result<STTReply>{
        let form = reqwest::blocking::multipart::Form::new().text("method", "large").file("speech", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        return Ok(client.post(format!("http://{}/api/services/whisper", self.ip_address.clone()))
        .multipart(form)
        .send()?.json()?);
    }

    pub fn vwav(&self) -> Result<()>{
        return Ok(());
    }

    pub fn srgan(&self) -> Result<()>{
        return Ok(());
    }

    pub fn llama(&self) -> Result<()>{
        return Ok(());
    }
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThalamusNodeStats {
    pub stt_tiny: i64,
    pub stt_basic: i64,
    pub stt_medium: i64,
    pub stt_large: i64,
    pub stt_score: i64,
    pub llama_tiny: i64,
    pub llama_basic: i64,
    pub llama_medium: i64,
    pub llama_large: i64,
    pub llama_score: i64,
    pub vwav_tiny: i64,
    pub vwav_basic: i64,
    pub vwav_medium: i64,
    pub vwav_large: i64,
    pub vwav_score: i64,
    pub srgan: i64,
}
impl ThalamusNodeStats {
    pub fn new() -> ThalamusNodeStats {
        ThalamusNodeStats { 
            stt_tiny: 0,
            stt_basic: 0,
            stt_medium: 0,
            stt_large: 0,
            stt_score: 0,
            llama_tiny: 0,
            llama_basic: 0,
            llama_medium: 0,
            llama_large: 0,
            llama_score: 0,
            vwav_tiny: 0,
            vwav_basic: 0,
            vwav_medium: 0,
            vwav_large: 0,
            vwav_score: 0, 
            srgan: 0,
        }
    }

    pub fn calculate(node: ThalamusNode) -> ThalamusNodeStats {


        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.stt_tiny("/home/kal/Documents/PixelCoda/sam/packages/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let tiny_stt = end_timestamp - start_timestamp;
        
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.stt_basic("/home/kal/Documents/PixelCoda/sam/packages/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let basic_stt = end_timestamp - start_timestamp;
        
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.stt_medium("/home/kal/Documents/PixelCoda/sam/packages/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let medium_stt = end_timestamp - start_timestamp;
        
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.stt_large("/home/kal/Documents/PixelCoda/sam/packages/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let large_stt = end_timestamp - start_timestamp;
        
        let stt_score = (tiny_stt + basic_stt + medium_stt + large_stt) / 4;

        // TODO: Calculate stats
        return ThalamusNodeStats { 
            stt_tiny: tiny_stt,
            stt_basic: basic_stt,
            stt_medium: medium_stt,
            stt_large: large_stt,
            stt_score: stt_score,
            llama_tiny: 0,
            llama_basic: 0,
            llama_medium: 0,
            llama_large: 0,
            llama_score: 0,
            vwav_tiny: 0,
            vwav_basic: 0,
            vwav_medium: 0,
            vwav_large: 0,
            vwav_score: 0, 
            srgan: 0,
        };
    }
}