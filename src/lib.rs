use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use local_ip_address::local_ip;
use local_ip_address::list_afinet_netifas;
use std::fs::File;
use rouille::Server;
use rouille::Response;
use simple_logger::SimpleLogger;
use std::path::Path;
use serde_json::{Value};
use std::sync::{Arc, Mutex};

extern crate rouille;

pub mod thalamus;
pub mod p2p;

// store application version as a const
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

// use error_chain::error_chain;
// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//         Json(serde_json::Error);
//     }
// }
use std::error::Error;


pub fn preinit(){
    // cls
    clearscreen::clear().unwrap();


    sudo::with_env(&["LIBTORCH", "LD_LIBRARY_PATH", "PG_DBNAME", "PG_USER", "PG_PASS", "PG_ADDRESS"]).unwrap();
    
    if Path::new("/opt/thalamus/").exists() {
        let touch_status = crate::thalamus::tools::touch("/opt/thalamus/output.log".to_string());
        if touch_status.is_ok() {
            SimpleLogger::new().with_colors(true).with_level(log::LevelFilter::Info).with_timestamps(true).with_output_file("/opt/thalamus/output.log".to_string()).init().unwrap();
        } else {
            SimpleLogger::new().with_colors(true).with_level(log::LevelFilter::Info).with_timestamps(true).init().unwrap();
        }
    } else {
        simple_logger::SimpleLogger::new().with_colors(true).with_level(log::LevelFilter::Info).with_timestamps(true).init().unwrap();
    }

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
}

pub fn init(){



    

    match crate::thalamus::setup::install_client(){
        Ok(_) => log::warn!("Installed thalamus client"),
        Err(e) => log::error!("Error installing thalamus client: {}", e),
    };

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
    pub nodes: Arc<Mutex<Vec<ThalamusNode>>>,
}
impl ThalamusClient {
    pub fn new() -> ThalamusClient {
        let x: Vec<ThalamusNode> = Vec::new();
        ThalamusClient { 
            nodes: Arc::new(Mutex::new(x)),
        }
    }

  
    // TODO: Automatically discover thalamus nodes
    pub fn discover(&mut self){

        let network_interfaces = list_afinet_netifas().unwrap();

        let nodex = Arc::clone(&self.nodes);

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
                            let mut nodes = nodex.lock().unwrap();
                            let existing_index = nodes.clone().iter().position(|r| r.pid == v.pid.to_string());
                            match existing_index {
                                Some(index) => {
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

    // TODO: Save client state to disk
    pub fn save(&self){
        std::fs::File::create("/opt/thalamusc/clients.json").expect("create failed");
        let j = serde_json::to_string(&self).unwrap();
        std::fs::write("/opt/thalamusc/clients.json", j).expect("Unable to write file");
    }

    // TODO: Load client state from disk
    pub fn load() -> Result<ThalamusClient, Box<dyn Error>>{
        let save_file = std::fs::read_to_string("/opt/thalamusc/clients.json");
        match save_file {
            Ok(save_data) => {
                let v: ThalamusClient = serde_json::from_str(&save_data)?;
                return Ok(v);
            },
            Err(e) => {
                log::error!("{}", format!("Unable to read file: {}", e));
                return Ok(ThalamusClient::new());
            }
        }
        return Ok(ThalamusClient::new());
    }

    pub fn select_optimal_node(&self, node_type: String) -> Result<ThalamusNode, Box<dyn Error>> {
        let mut nodex = self.nodes.lock().unwrap();
        let mut nodes = nodex.clone();
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



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionReply {
    pub version: String,
    pub pid: String,
}
pub fn fetch_version(host: &str) -> Result<VersionReply, Box<dyn Error>> {
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

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;

        let new_file_name = format!("{}.{}", timestamp, extension);

        let form = reqwest::blocking::multipart::Form::new().text("filename", new_file_name).file("input_file", tmp_file_path.as_str())?;

        let client = reqwest::blocking::Client::builder().timeout(None).build()?;

        let bytes = client.post(format!("http://{}/api/services/image/srgan", self.ip_address.clone()))
        .multipart(form)
        .send()?.bytes()?;

        return Ok(bytes.to_vec());
    }

    pub fn llama(&self) -> Result<(), Box<dyn Error>>{
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
        }
    }

    pub fn calculate(node: ThalamusNode) -> ThalamusNodeStats {


        log::warn!("Calculating stats for node {}.....", node.pid);


        log::warn!("{}: Running STT Tiny test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.stt_tiny("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let tiny_stt = end_timestamp - start_timestamp;
        log::warn!("{}: STT Tiny test complete in {} miliseconds", node.pid, tiny_stt);
        
        log::warn!("{}: Running STT Base test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.stt_base("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let basic_stt = end_timestamp - start_timestamp;
        log::warn!("{}: STT Base test complete in {} miliseconds", node.pid, basic_stt);
        
        log::warn!("{}: Running STT Medium test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.stt_medium("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let medium_stt = end_timestamp - start_timestamp;
        log::warn!("{}: STT Medium test complete in {} miliseconds", node.pid, medium_stt);

        log::warn!("{}: Running STT Large test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.stt_large("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let large_stt = end_timestamp - start_timestamp;
        log::warn!("{}: STT Large test complete in {} miliseconds", node.pid, large_stt);
        
        let stt_score = (tiny_stt + basic_stt + medium_stt + large_stt) / 4;


        log::warn!("{}: Running VWAV Tiny test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.vwav_tiny("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let vwav_tiny = end_timestamp - start_timestamp;
        log::warn!("{}: VWAV Tiny test complete in {} miliseconds", node.pid, vwav_tiny);
        
        log::warn!("{}: Running VWAV Base test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.vwav_base("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let vwav_base = end_timestamp - start_timestamp;
        log::warn!("{}: VWAV Base test complete in {} miliseconds", node.pid, vwav_base);
        
        log::warn!("{}: Running VWAV Medium test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.vwav_medium("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let vwav_medium = end_timestamp - start_timestamp;
        log::warn!("{}: VWAV Medium test complete in {} miliseconds", node.pid, vwav_medium);

        log::warn!("{}: Running VWAV Large test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.vwav_large("/opt/thalamusc/test.wav".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let vwav_large = end_timestamp - start_timestamp;
        log::warn!("{}: VWAV Large test complete in {} miliseconds", node.pid, vwav_large);

        let vwav_score = (vwav_tiny + vwav_base + vwav_medium + vwav_large) / 4;

        log::warn!("{}: Running SRGAN test...", node.pid);
        let start_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let stt = node.srgan("/opt/thalamusc/test.jpg".to_string()).unwrap();
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let srgan = end_timestamp - start_timestamp;
        log::warn!("{}: SRGAN test complete in {} miliseconds", node.pid, srgan);

        // let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;

        // let mut file = std::fs::File::create(format!("/opt/thalamusc/{}", timestamp))?;
        // let mut content =  Cursor::new(bytes);
        // std::io::copy(&mut content, &mut file)?;


        // TODO: Calculate stats
        return ThalamusNodeStats { 
            stt_tiny: tiny_stt,
            stt_base: basic_stt,
            stt_medium: medium_stt,
            stt_large: large_stt,
            stt_score: stt_score,
            llama_tiny: 0,
            llama_basic: 0,
            llama_medium: 0,
            llama_large: 0,
            llama_score: 0,
            vwav_tiny: vwav_tiny,
            vwav_base: vwav_base,
            vwav_medium: vwav_medium,
            vwav_large: vwav_large,
            vwav_score: vwav_score, 
            srgan: srgan,
        };
    }
}