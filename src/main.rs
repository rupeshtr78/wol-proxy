use wol::WolRequest;
mod wol;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://");
    // start http server with actix-web
    HttpServer::new(|| App::new().route("/send_wol", web::post().to(send_wol_request)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// fn main() {
//     check_wol();
// }

async fn send_wol_request(wol_req: web::Json<WolRequest>) -> impl Responder {
    let mac_address = &wol_req.mac_address;
    let bind_addr = &wol_req.bind_addr;
    let broadcast_addr = &wol_req.broadcast_addr;

    let request = WolRequest::new(mac_address, bind_addr, broadcast_addr);

    match wol::send_wol(&request) {
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        Ok(_) => HttpResponse::Ok().body("Magic packet sent successfully"),
    }
}

fn check_wol() {
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

    // call wol::send_wol as actix-web post request
    // the post request should take a json object with WolRequest fields
}
