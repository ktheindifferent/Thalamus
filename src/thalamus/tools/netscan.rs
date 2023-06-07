use std::sync::mpsc::channel;
use std::thread;
use port_scanner::scan_port_addr;
use std::sync::{Arc, Mutex};

pub fn scan_bulk(base_ip: &str, port: &str, cidr: &str) -> Result<Vec<String>, crate::thalamus::tools::Error> {
    let mut handles = vec![];

    let ik: Vec<String> = Vec::new();
    let matched_ips = Arc::new(Mutex::new(ik));
    let ips = crate::thalamus::tools::cidr::get_cidr_mask_range_from_port(port.to_string(), base_ip.to_string(), cidr.to_string());

    let (tx, _rx) = channel();
    let tx = Arc::new(Mutex::new(tx.clone()));
    for ip in ips.clone() {
        let txc = Arc::clone(&tx);
        let mipc = Arc::clone(&matched_ips);
        let handle = thread::spawn(move || {
            log::warn!("Scanning IP: {}", ip);
            if scan_port_addr(ip.clone()){
                let txl = txc.lock().unwrap();
                let mut mipl = mipc.lock().unwrap();
                mipl.push(ip.clone());
                txl.send(ip).expect("channel will be there waiting for the pool");
                std::mem::drop(txl);
                std::mem::drop(mipl);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    
    return Ok(matched_ips.lock().unwrap().clone());
}
