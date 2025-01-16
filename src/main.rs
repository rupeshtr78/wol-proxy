#![warn(unused_crate_dependencies)]
use ::actix_governor::{Governor, GovernorConfigBuilder};
use ::actix_web::http::StatusCode;
use ::log::debug;
use actix_web::{
    http::header::ContentType, middleware, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use log;
use rustls::pki_types::IpAddr;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use wol::WolRequest;

mod openssltls;
mod security;
mod wol;

const COOKIE_NAME: &str = "wol-cookie";
const SERVER_CERT: &str = "/home/rupesh/aqrtr/security/ssl/clients/wildcard-rupesh/client.crt";
const SERVER_KEY: &str = "/home/rupesh/aqrtr/security/ssl/clients/wildcard-rupesh/client.key";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    // load env variables from .env file
    dotenvy::dotenv().ok();

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

    // start http server with actix-web
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Governor::new(&governor_conf))
            // .route("/wol", web::post().to(send_wol_request))
            .service(send_wol_request)
            .service(status)
            .service(index)
    });

    if tls == "false" {
        log::info!("{}", format!("Starting wol http server at port: {}", port));
        return server.bind(format!("0.0.0.0:{}", port))?.run().await;
    }

    // @todo issue with ios client when using my own root ca with rustls
    // let tls_config = security::get_server_config(&server_cert, &server_key)?;

    let builder = openssltls::get_server_certs(&server_cert, &server_key);
    let builder = match builder {
        Ok(builder) => builder,
        Err(e) => {
            log::error!("Error loading server certificates: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error loading server certificates",
            ));
        }
    };

    let tls_port = port.parse::<u16>().unwrap_or(8443);
    log::info!("{}", format!("Starting wol https server at port: {}", port));
    server
        // .bind_rustls_0_23(("0.0.0.0", tls_port), server_config)?
        .bind_openssl(("0.0.0.0", tls_port), builder)?
        .tls_handshake_timeout(Duration::from_secs(10))
        .workers(4)
        .run()
        .await
}

#[actix_web::post("/wol")]
async fn send_wol_request(wol_req: web::Json<WolRequest>, req: HttpRequest) -> impl Responder {
    if !security::verify_cookie(req, COOKIE_NAME) {
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

#[actix_web::get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"
        <html>
            <head>
                <title>WOL Server</title>
            </head>
            <body>
                <h1>WOL Server</h1>
                <p>Wake on LAN server</p>
            </body>
        </html>
        "#,
    )
}

#[actix_web::post("/status")]
async fn status(req: HttpRequest, ip_addr: web::Json<IpAddress>) -> impl Responder {
    if !security::verify_cookie(req, COOKIE_NAME) {
        return HttpResponse::Unauthorized().body(format!(
            "Status: {:?}, Unauthorized request",
            StatusCode::UNAUTHORIZED
        ));
    }

    // let r = req.clone();
    // debug!("Request: {:?}", r);

    if is_port_open(&ip_addr, Duration::from_secs(5)) {
        return HttpResponse::Ok().body(format!(
            "Server is Online port {} is open on {}",
            ip_addr.port, ip_addr.ip
        ));
    }
    HttpResponse::Ok().body(format!(
        "Server is Offline port {} is closed on {}",
        ip_addr.port, ip_addr.ip
    ))
}

#[derive(serde::Deserialize)]
struct IpAddress {
    ip: String,
    port: String,
}

fn is_port_open(ssh: &IpAddress, timeout: Duration) -> bool {
    let addr = format!("{}:{}", ssh.ip, ssh.port)
        .parse::<SocketAddr>()
        .unwrap();
    log::debug!("Checking if port {} is open on {}", ssh.port, ssh.ip);
    TcpStream::connect_timeout(&addr, timeout).is_ok()
}
