use ::actix_web::http::StatusCode;
use actix_web::cookie::{time::Duration, Key};
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use log;
use wol::WolRequest;
mod wol;
use actix_identity::IdentityMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_session::{config::PersistentSession, SessionMiddleware};

const COOKIE_NAME: &str = "wol-cookie";
const COOKIE_DURATION: Duration = Duration::minutes(1);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");

    let port = std::env::var("WOL_PORT").unwrap_or("8090".to_string());
    let secret_key = std::env::var("COOKIE_SECRET_KEY")
        .expect("COOKIE_SECRET_KEY environment variable must be set");
    let secret_key = create_secret_key(&secret_key);

    // start http server with actix-web
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/wol", web::post().to(send_wol_request))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name(COOKIE_NAME.to_string())
                    .cookie_secure(false)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::seconds(100)),
                    )
                    .build(),
            )
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

fn create_secret_key(key: &str) -> Key {
    Key::derive_from(key.as_bytes())
}
