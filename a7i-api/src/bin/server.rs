#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let _ = a7i_api::server::server("0.0.0.0:80").await;
}
