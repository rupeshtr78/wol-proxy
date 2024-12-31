#![warn(unused_crate_dependencies)]
use ::actix_web::http::StatusCode;
use ::log::debug;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log;
use wol::WolRequest;
mod wol;
use hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

const COOKIE_NAME: &str = "wol-cookie";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    log::info!("starting HTTP server at http://localhost:8080");

    let port = std::env::var("WOL_PORT").unwrap_or("8090".to_string());

    // start http server with actix-web
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // .route("/wol", web::post().to(send_wol_request))
            .service(send_wol_request)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

#[actix_web::post("/wol")]
async fn send_wol_request(wol_req: web::Json<WolRequest>, req: HttpRequest) -> impl Responder {
    if !verify_cookie(req) {
        return HttpResponse::Unauthorized().body(format!(
            "Status: {:?}, Unauthorized request",
            StatusCode::UNAUTHORIZED
        ));
    }

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

// todo: implement cookie verification better way
fn verify_cookie(req: HttpRequest) -> bool {
    // Check if the cookie is present
    let cookie = match req.cookie(COOKIE_NAME) {
        Some(cookie) => cookie,
        None => {
            log::error!("Cookie not found");
            return false;
        }
    };

    // Validate the cookie value
    match is_valid_cookie(cookie.value()) {
        Ok(true) => {
            return true;
        }
        Ok(false) | Err(_) => {
            return false;
        }
    }
}

// @TODO: Implement cookie validation
fn is_valid_cookie(cookie_value: &str) -> Result<bool, std::env::VarError> {
    if cookie_value.is_empty() {
        return Ok(false);
    }
    // check if first characters before . are equal to secret value
    let parts: Vec<&str> = cookie_value.split('.').collect();
    if parts.len() != 2 {
        log::error!("Invalid cookie format");
        return Ok(false);
    }

    debug!("Cookie parts: {:?}", parts);
    let (value, signature) = (parts[0], parts[1]);
    // validate signature using key

    let secret_key = std::env::var("COOKIE_SECRET_KEY")?;
    let cookie_value = std::env::var("COOKIE_SECRET_VALUE")?;

    // Recompute the HMAC signature
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.trim().as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(cookie_value.trim().as_bytes());
    let computed_signature = mac.finalize().into_bytes();

    // Compare the computed signature with the provided signature
    let computed_signature_hex = hex::encode(computed_signature);

    // Compare the computed signature with the provided signature
    if !(computed_signature_hex.trim() == signature.trim()) {
        log::error!("Invalid signature");
        debug!("Computed signature: {}", computed_signature_hex);
        debug!("Provided signature: {}", signature);
        return Ok(false);
    }

    // Compare the value with the secret key
    if value == cookie_value {
        Ok(true)
    } else {
        debug!(
            "{}",
            format!("Invalid cookie: {} secret {}", value, secret_key)
        );
        Ok(false)
    }
}
