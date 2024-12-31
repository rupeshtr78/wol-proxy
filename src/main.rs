use ::actix_web::http::StatusCode;
use wol::WolRequest;
mod wol;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");

    // start http server with actix-web
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/wol", web::post().to(send_wol_request))
    })
    .bind("10.0.0.191:8090")?
    .run()
    .await
}

async fn send_wol_request(wol_req: web::Json<WolRequest>) -> impl Responder {
    let mac_address = &wol_req.mac_address;
    let bind_addr = &wol_req.bind_addr;
    let broadcast_addr = &wol_req.broadcast_addr;

    let request = WolRequest::new(mac_address, bind_addr, broadcast_addr);

    match wol::send_wol(&request) {
        Err(e) => HttpResponse::InternalServerError().body(format!(
            "Status: {:?}, Error: {}",
            StatusCode::INTERNAL_SERVER_ERROR,
            e
        )),
        Ok(_) => HttpResponse::Ok().body(format!(
            "Status: {:?}, Magic packet sent successfully",
            StatusCode::OK
        )),
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
