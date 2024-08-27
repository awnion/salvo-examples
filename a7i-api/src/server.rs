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

    let rustls_config = RustlsConfig::new(
        Keycert::new()
            .cert(include_bytes!("../certs/cert.pem").as_ref())
            .key(include_bytes!("../certs/key.pem").as_ref()),
    );

    let acceptor = TcpListener::new("127.0.0.1:8081")
        .rustls(rustls_config.clone())
        // .join(TcpListener::new("127.0.0.1:8081"))
        .join(QuinnListener::new(rustls_config, "127.0.0.1:8081"))
        .bind()
        .await;
    Server::new(acceptor).serve(service).await;
}
