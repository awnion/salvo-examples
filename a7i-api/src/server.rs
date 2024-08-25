use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "hello"
}

pub async fn server(addr: &str) -> () {
    let router = Router::new().get(hello).post(hello);
    let service = Service::new(router).hoop(Logger::new());

    let rustls_config = RustlsConfig::new(
        Keycert::new()
            .key_from_path("./a7i-api/certs/key.pem")
            .expect("key.pem")
            .cert_from_path("./a7i-api/certs/cert.pem")
            .expect("cert.pem"),
    );

    let acceptor = TcpListener::new(addr.to_string())
        .join(QuinnListener::new(rustls_config, "[::]:4444"))
        .bind()
        .await;
    Server::new(acceptor).serve(service).await;
}
