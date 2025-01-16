#![allow(unused_imports)]
use actix_web::HttpRequest;
use anyhow::bail;
use anyhow::{Context, Result};
use hmac::{Hmac, Mac};
use log::info;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::server::WebPkiClientVerifier;
use rustls::{RootCertStore, ServerConfig};
use rustls_native_certs::load_native_certs;
use rustls_pemfile::certs;
use sha2::Sha256;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

/// Get server config
/// Args:
///    server_cert: &String - Path to server cert
///   server_key: &String - Path to server key
/// Returns:
///   Result<ServerConfig> - ServerConfig
/// Errors:
///  Failed to get server certs
#[allow(dead_code)]
pub fn get_server_config(server_cert: &String, server_key: &String) -> Result<ServerConfig> {
    // get system certs
    let mut roots = RootCertStore::empty();

    let root_certs = load_native_certs();
    if root_certs.errors.len() > 0 {
        info!("Failed to load native certs");
    }
    for cert in root_certs.certs {
        roots.add(cert).context("Failed to add cert to store")?;
    }

    // get server certs and private key
    let server_certs = get_certs(&server_cert).context("Failed to get server certs")?;
    let server_key = get_key(&server_key).context("Failed to get server key")?;

    // create server config
    let client_auth = WebPkiClientVerifier::builder(Arc::new(roots))
        .build()
        .context("Failed to create client verifier")?;

    let tls_config = ServerConfig::builder()
        .with_client_cert_verifier(client_auth)
        .with_single_cert(server_certs, server_key)
        .context("Failed to create server config")?;

    Ok(tls_config)
}

/// Get certs with 'static lifetime
/// Args:
///  path: &str - Path to certs
/// Returns:
/// Result<Vec<CertificateDer<'static>>> - Vec<CertificateDer<'static>>
#[allow(dead_code)]
fn get_certs(path: &str) -> Result<Vec<CertificateDer<'static>>> {
    log::debug!("Getting Server Certs!");

    let cert_path = Path::new(path);
    if !cert_path.exists() {
        log::error!("File not found: {}", path);
        return Err(anyhow::anyhow!("File not found: {}", path));
    }

    let cert_file = File::open(path).context("Failed to open server cert file")?;
    let mut reader = BufReader::new(cert_file);
    let certs: Vec<CertificateDer<'_>> = certs(&mut reader)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to read server cert file")?;

    if certs.is_empty() {
        return Err(anyhow::anyhow!("No certs found in file"));
    }

    // Convert to owned certs with 'static lifetime
    let static_certs = certs.iter().map(|cert| cert.clone().into_owned()).collect();

    log::debug!("Got Server Certs: {:?}", path);
    Ok(static_certs)
}

/// Get private key with 'static lifetime
/// Args:
///  path: &str - Path to private key
/// Returns:
/// Result<PrivateKeyDer> - PrivateKeyDer<'static>
#[allow(dead_code)]
fn get_key(path: &str) -> Result<PrivateKeyDer<'static>> {
    log::debug!("Getting Server private Key!");

    let key_path = Path::new(path);
    if !key_path.exists() {
        bail!("File not found: {}", path);
    }

    let key_file = File::open(path).context("Failed to open server key file")?;
    let mut reader = BufReader::new(key_file);
    let keys = rustls_pemfile::private_key(&mut reader)
        .into_iter()
        .map(|opt| opt.ok_or_else(|| anyhow::anyhow!("Failed to parse private key"))) // Convert Option to Result
        .collect::<Result<Vec<_>, _>>() // Collect into Result<Vec<_>, _>
        .context("Failed to read server key file")?;

    if keys.is_empty() || keys.len() > 1 {
        bail!("No key found in file");
    }

    match keys.into_iter().next() {
        Some(key) => Ok(key.clone_key()), // Move the key out of the vector
        None => Err(anyhow::anyhow!("No key found in file")),
    }
}

// todo: implement cookie verification better way kind of a hack now to get it going
pub fn verify_cookie(req: HttpRequest, cookie_name: &str) -> bool {
    // Check if the cookie is present
    let cookie = match req.cookie(cookie_name) {
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

    log::debug!("Cookie parts: {:?}", parts);
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
        log::debug!("Computed signature: {}", computed_signature_hex);
        log::debug!("Provided signature: {}", signature);
        return Ok(false);
    }

    // Compare the value with the secret key
    if value == cookie_value {
        Ok(true)
    } else {
        log::debug!(
            "{}",
            format!("Invalid cookie: {} secret {}", value, secret_key)
        );
        Ok(false)
    }
}
