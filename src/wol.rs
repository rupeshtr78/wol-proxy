use anyhow::{Context, Result};
use mac_address::MacAddress;
use std::net::{IpAddr, UdpSocket};

const MAC_ADDR_SIZE: usize = 6;
const BIND_PORT: u16 = 0;
const WOL_PORT: u16 = 9;
const DEFAULT_BIND_ADDR: &str = "0.0.0.0";
const DEFAULT_BROADCAST_ADDR: &str = "255.255.255.255";

pub struct WolRequest {
    pub mac_address: String,
    pub bind_addr: Option<String>,
    pub broadcast_addr: Option<String>,
}

pub fn parse_ip_addr(addr: &str) -> Result<IpAddr> {
    addr.parse().context("Failed to parse IP address")
}

impl WolRequest {
    pub fn new(mac_address: String) -> Self {
        Self {
            mac_address,
            bind_addr: None,
            broadcast_addr: None,
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

    // magic packet, 102 bytes
    let mut magic_packet = vec![0; 102];
    // first 6 bytes are 0xff
    for i in 0..6 {
        magic_packet[i] = 0xff;
    }
    // followed by 16 times of mac address
    for i in 0..16 {
        for j in 0..MAC_ADDR_SIZE {
            magic_packet[6 + i * MAC_ADDR_SIZE + j] = mac_addr.bytes()[j];
        }
    }

    let socket = UdpSocket::bind((bind_addr, BIND_PORT)).unwrap();
    socket
        .set_broadcast(true)
        .expect("set_broadcast call failed");

    socket
        .send_to(&magic_packet, (bcast_addr, WOL_PORT))
        .unwrap();

    Ok(())
}
