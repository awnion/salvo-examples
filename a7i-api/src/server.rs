use std::process::exit;

use rcgen::CertifiedKey;
use salvo::conn::rustls::Keycert;
use salvo::conn::rustls::RustlsConfig;
use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "hello"
}

pub fn router() -> Router {
    Router::new().get(hello).post(hello)
}

pub async fn server(addr: &str) -> () {
    let service = Service::new(router()) //
        .hoop(Logger::new())
        .hoop(ForceHttps::new());

    // generate self-signed keys
    let CertifiedKey { cert, key_pair } = rcgen::generate_simple_self_signed([
        //
        "localhost".into(),
        "::1".into(),
        "127.0.0.1".into(),
    ])
    .expect("generate self-signed certs");

    // tracing::warn!("cert: {}", key.cert.pem());
    // tracing::warn!("key: {}", key.key_pair.serialize_pem());

    let keycert = Keycert::new().cert(cert.pem()).key(key_pair.serialize_pem());
    let rustls_config = RustlsConfig::new(keycert);

    let acceptor = TcpListener::new("127.0.0.1:8081")
        .rustls(rustls_config.clone())
        // .join(TcpListener::new("127.0.0.1:8081"))
        .join(QuinnListener::new(rustls_config, "127.0.0.1:8081"))
        .bind()
        .await;
    Server::new(acceptor).serve(service).await;
}
