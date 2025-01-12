use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use anyhow::bail;
use anyhow::{Context, Result};
use log::{info, warn};
use rustls::pki_types::CertificateDer;
use rustls::pki_types::PrivateKeyDer;
use rustls::server::WebPkiClientVerifier;
use rustls::{RootCertStore, ServerConfig};
use rustls_native_certs::load_native_certs;
use rustls_pemfile::certs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

pub fn get_server_config(server_cert: &String, server_key: &String) -> Result<ServerConfig> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

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

    let server_certs: Vec<CertificateDer<'static>> = server_certs
        .iter()
        .map(|cert| cert.clone().into_owned())
        .collect();
    let server_key: PrivateKeyDer<'static> = server_key.clone_key();

    let client_auth = WebPkiClientVerifier::builder(Arc::new(roots))
        .build()
        .context("Failed to create client verifier")?;

    let tls_config = ServerConfig::builder()
        .with_client_cert_verifier(client_auth)
        .with_single_cert(server_certs, server_key)
        .context("Failed to create server config")?;

    Ok(tls_config)
}

fn get_certs(path: &str) -> Result<Vec<CertificateDer<'_>>> {
    println!("Getting Certs!");

    let cert_path = Path::new(path);
    if !cert_path.exists() {
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

    Ok(certs)
}

fn get_key(path: &str) -> Result<PrivateKeyDer<'_>> {
    println!("Getting Private Key!");

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
        Some(key) => Ok(key), // Move the key out of the vector
        None => Err(anyhow::anyhow!("No key found in file")),
    }
}
