use dawn::dawn_server::DawnServer;

#[tokio::main]
async fn main() -> Result<(), String> {
    DawnServer::new().serve().await
}
