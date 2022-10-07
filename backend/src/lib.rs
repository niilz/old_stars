#[macro_use]
extern crate diesel;

use rocket::config::{CipherSuite, TlsConfig};

pub mod db;
pub mod model;
pub mod schema;

pub fn tls_config(cert_path: &str, privat_key: &str) -> TlsConfig {
    TlsConfig::from_paths(cert_path, privat_key)
        .with_ciphers(CipherSuite::TLS_V13_SET)
        .with_preferred_server_cipher_order(true)
}
