use wol::WolRequest;
mod wol;

fn main() {
    // let request = WolRequest::new("F4:93:9F:F4:04:5B".to_string());
    let request = WolRequest {
        mac_address: "F4:93:9F:F4:04:5B".to_string(),
        bind_addr: Some("0.0.0.0".to_string()),
        broadcast_addr: Some("255.255.255.255".to_string()),
    };
    let result = wol::send_wol(&request);
    match result {
        Err(e) => eprintln!("Error: {}", e),
        Ok(_) => println!("Magic packet sent successfully"),
    }
}
