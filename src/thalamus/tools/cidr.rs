// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

// extern crate ipnet;
use std::net::{Ipv4Addr};
// use std::str::FromStr;
// use ipnet::{IpNet, Ipv4Net, Ipv6Net};

// use std::fs::File;
// use std::io::{self, BufRead};

pub fn get_cidr_mask_range_from_port(port: String, raw_ip: String, cidr: String) -> Vec<String> {
    //println!("get_cidr_mask_range_from_port: {}, {}, {}", port, raw_ip, cidr);
	let mut local_network_range: Vec<String> = Vec::new();
	let ips = get_each_ip_in_range(&(raw_ip + &cidr));
    //println!("ips: {:?}", ips);
	match ips {
		Some(ips) => {
			for ip in ips {
				local_network_range.push(format!("{}:{}", Ipv4Addr::from(ip), &port));
			}
		}
		None => {}
	}
    //println!("local_network_range: {:?}", local_network_range);

	local_network_range
}






struct IpRange {
    first: u32,
    last: u32,
}

fn set_bit(original: u32, bit: u32) -> u32 {
    let mask = 1 << bit;
    original | mask
}

fn get_ip_range(cidr: &str) -> Option<IpRange> {
    let ip_and_mask = cidr.split('/').collect::<Vec<&str>>();
    if ip_and_mask.len() < 2 {
        return None;
    }
    let ip = ip_and_mask[0]; // IP Address
    let mask: u32 = ip_and_mask[1].parse::<u32>().unwrap(); // Subnet mask
    let addr: Ipv4Addr = ip.parse().unwrap(); // IPv4

    let mut ip_mask_long: u32 = 0;
    let mut inverse_ip_mask_long: u32 = 0;
    for i in 0..32 {
        if i < mask {
            ip_mask_long = set_bit(ip_mask_long, 31 - i);
        } else {
            inverse_ip_mask_long = set_bit(inverse_ip_mask_long, 31 - i)
        }
    }

    //println!("{} {}", ip_mask_long, Ipv4Addr::from(ip_mask_long));
    //println!("{} {}", inverse_ip_mask_long, Ipv4Addr::from(inverse_ip_mask_long));
    let ip_long = u32::from(addr); // u32 of the IP address
                                   //println!("{}", Ipv4Addr::from(ip_long));

    let network = ip_long & ip_mask_long;
    let start = network + 1; // ignore network IP, i.e. 192.168.2.0
    let end = (network | inverse_ip_mask_long) - 1; // ignore broadcast IP, i.e. 192.168.2.255

    let res = IpRange {
        first: start,
        last: end,
    };
    Some(res)
}

// fn file_to_string_vec(filename: &str) -> Vec<String> {
//     let file = File::open(filename).unwrap();
//     let buf = io::BufReader::new(file);
//     let results: Vec<String> = buf
//         .lines()
//         .map(|l| l.expect("Could not parse line"))
//         .collect::<Vec<String>>();
//     return results;
// }

fn get_each_ip_in_range(cidr: &str) -> Option<Vec<u32>> {
    //println!("get_each_ip_in_range: {}", cidr);
    let range = get_ip_range(cidr);
   
    match range {
        Some(range) => {
            //println!("range_first: {}", Ipv4Addr::from(range.first));
            //println!("range_last: {}", Ipv4Addr::from(range.last));

            let mut res: Vec<u32> = Vec::new();
            for ip in range.first..range.last {
                res.push(ip);
            }
            for ip in range.last..range.first {
                res.push(ip);
            }
            Some(res)
        }
        None => {
            None
        }
    }
}