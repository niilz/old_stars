#[macro_use]
extern crate diesel;

use rocket::config::{CipherSuite, Config, TlsConfig};

pub mod db;
pub mod model;
pub mod schema;

pub fn tls_config(cert_path: &str, privat_key: &str) -> Config {
    let tls_config = TlsConfig::from_paths(cert_path, privat_key)
        .with_ciphers(CipherSuite::TLS_V13_SET)
        .with_preferred_server_cipher_order(true);

    Config {
        tls: Some(tls_config),
        port: 443,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::tls_config;

    #[test]
    fn non_existing_cert_path_gives_empty_config() {
        let path = "./does/not/exist.cert";

        let empty_config = tls_config(path, path);

        assert!(empty_config.tls_enabled());

        let is_left = empty_config.tls.as_ref().unwrap().certs().is_left();
        assert!(is_left);
        let is_right = empty_config.tls.unwrap().certs().is_right();
        assert!(!is_right);
    }
}
