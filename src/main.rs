#![warn(unused_crate_dependencies)]
use ::actix_governor::{Governor, GovernorConfigBuilder};
use ::actix_web::http::StatusCode;
use ::log::debug;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log;
use std::time::Duration;
use wol::WolRequest;

use hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

mod security;
mod wol;

const COOKIE_NAME: &str = "wol-cookie";
const SERVER_CERT: &str = "/home/rupesh/aqrtr/security/ssl/clients/wildcard-rupesh/client.crt";
const SERVER_KEY: &str = "/home/rupesh/aqrtr/security/ssl/clients/wildcard-rupesh/client.key";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let port = std::env::var("WOL_PORT").unwrap_or("9888".to_string());
    let tls = std::env::var("WOL_TLS").unwrap_or("true".to_string());

    let server_cert = std::env::var("WOL_SERVER_CERT").unwrap_or(SERVER_CERT.to_string());
    let server_key = std::env::var("WOL_SERVER_KEY").unwrap_or(SERVER_KEY.to_string());

    // governor configuration to limit requests
    let governor_conf = GovernorConfigBuilder::default()
        .requests_per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    let tls_config = security::get_server_config(&server_cert, &server_key);
    let server_config = match tls_config {
        Ok(server_config) => {
            log::info!("TLS Config created successfully");
            server_config
        }
        Err(e) => {
            log::error!("Failed to create TLS Config: {}", e);
            return Ok(());
        }
    };

    // start http server with actix-web
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Governor::new(&governor_conf))
            // .route("/wol", web::post().to(send_wol_request))
            .service(send_wol_request)
            .service(index)
    });

    if tls == "false" {
        log::info!("{}", format!("Starting wol http server at port: {}", port));
        return server.bind(format!("0.0.0.0:{}", port))?.run().await;
    }

    log::info!("{}", format!("Starting wol https server at port: {}", port));
    server
        .bind_rustls_0_23(("0.0.0.0", 8443), server_config)?
        .tls_handshake_timeout(Duration::from_secs(10))
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

// todo: implement cookie verification better way kind of a hack now to get it going
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

#[actix_web::get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "WOL TLS Server!"
}
