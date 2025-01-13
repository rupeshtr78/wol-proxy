use openssl::{
    pkey::{PKey, Private},
    ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod},
};

use anyhow::Result;
use log::debug;
pub fn get_server_certs(server_cert: &str, server_key: &str) -> Result<SslAcceptorBuilder> {
    debug!("Loading server certificates");
    let mut builder: SslAcceptorBuilder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file(server_key, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(server_cert)?;
    builder.check_private_key()?;

    log::info!("Openssl Server certificates loaded successfully");
    Ok(builder)
}
