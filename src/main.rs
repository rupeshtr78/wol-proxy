use ::actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use log;
use wol::WolRequest;
mod wol;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");

    let port = std::env::var("WOL_PORT").unwrap_or("8090".to_string());

    // start http server with actix-web
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/wol", web::post().to(send_wol_request))
    })
    .bind(format!("0.0.0.0:{}", port))?
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
