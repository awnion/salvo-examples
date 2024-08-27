#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let _ = a7i_api::server::server("0.0.0.0:80").await;
}

#[cfg(test)]
mod tests {
    use salvo::prelude::*;
    use salvo::test::ResponseExt;
    use salvo::test::TestClient;

    #[tokio::test]
    async fn test_basic() {
        let service = Service::new(a7i_api::server::router());

        let content = TestClient::get("https://localhost:8081/")
            .send(&service)
            .await
            .take_string()
            .await
            .unwrap();
        assert!(content.contains("hello"));
    }
}
