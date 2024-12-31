use anyhow::{Context, Result};
use mac_address::MacAddress;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, UdpSocket};

const MAC_ADDR_SIZE: usize = 6;
const BIND_PORT: u16 = 0;
const WOL_PORT: u16 = 9;
const DEFAULT_BIND_ADDR: &str = "0.0.0.0";
const DEFAULT_BROADCAST_ADDR: &str = "255.255.255.255";

#[derive(Serialize, Deserialize)]
pub struct WolRequest {
    pub mac_address: String,
    pub bind_addr: Option<String>,
    pub broadcast_addr: Option<String>,
}

pub fn parse_ip_addr(addr: &str) -> Result<IpAddr> {
    addr.parse().context("Failed to parse IP address")
}

impl WolRequest {
    pub fn new(
        mac_address: &String,
        bind_addr: &Option<String>,
        broadcast_addr: &Option<String>,
    ) -> Self {
        WolRequest {
            mac_address: mac_address.clone(),
            bind_addr: bind_addr.clone(),
            broadcast_addr: broadcast_addr.clone(),
        }
    }

    pub fn get_mac_address(&self) -> Result<MacAddress> {
        self.mac_address
            .parse()
            .context("Failed to parse MAC address")
    }

    pub fn get_bind_addr(&self) -> Result<IpAddr> {
        match &self.bind_addr {
            Some(addr) => parse_ip_addr(&addr),
            None => parse_ip_addr(&DEFAULT_BIND_ADDR),
        }
    }

    pub fn get_broadcast_addr(&self) -> Result<IpAddr> {
        match &self.broadcast_addr {
            Some(addr) => parse_ip_addr(&addr),
            None => parse_ip_addr(&DEFAULT_BROADCAST_ADDR),
        }
    }
}

pub fn send_wol(data: &WolRequest) -> Result<()> {
    let mac_addr: MacAddress = data
        .get_mac_address()
        .with_context(|| format!("Failed to get MAC address: {}", data.mac_address))?;

    let bind_addr: IpAddr = data.get_bind_addr().with_context(|| {
        format!(
            "Failed to get bind address for: {:?}",
            &data.bind_addr.as_ref().expect("bind address")
        )
    })?;

    let bcast_addr: IpAddr = data.get_broadcast_addr().with_context(|| {
        format!(
            "Failed to get broadcast address for: {:?}",
            &data.broadcast_addr.as_ref().expect("broadcast address")
        )
    })?;

    let magic_packet = create_magic_packet(&mac_addr.to_string());

    let socket = UdpSocket::bind((bind_addr, BIND_PORT))
        .with_context(|| format!("Failed to bind UDP socket to: {:?}", &bind_addr.to_string()))?;

    socket
        .set_broadcast(true)
        .with_context(|| "Failed to set socket to broadcast mode")?;

    match socket.send_to(&magic_packet, (bcast_addr, WOL_PORT)) {
        Err(e) => {
            let error = format!("Failed to send magic packet: {}", e);
            return Err(anyhow::anyhow!(error));
        }
        Ok(_) => {}
    }

    Ok(())
}

/// Wake-on-LAN (WoL) magic packet, the first 6 bytes are always "FF FF FF FF FF FF" (hexadecimal),
/// which translates to six repetitions of the value 255, essentially a pattern of all "ones" in binary;
/// this is considered the "magic" part of the packet that identifies it as a WoL signal.
/// MAC address repetition:
/// After the initial "FF FF FF FF FF FF" sequence,
/// the packet contains 16 repetitions of the target computer's MAC address,
/// which is how the specific device is identified to wake up
/// for a total of 102 bytes.
fn create_magic_packet(mac_address: &str) -> Vec<u8> {
    let mut magic_packet = vec![0; 102];
    for i in 0..6 {
        magic_packet[i] = 0xff;
    }

    let mac_address_bytes: Vec<u8> = mac_address
        .split(':')
        .map(|byte| u8::from_str_radix(byte, 16).unwrap())
        .collect();

    for i in 0..16 {
        for j in 0..MAC_ADDR_SIZE {
            magic_packet[6 + i * MAC_ADDR_SIZE + j] = mac_address_bytes[j];
        }
    }

    magic_packet
}
