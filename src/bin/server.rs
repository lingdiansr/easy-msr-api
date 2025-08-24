use rust_wrapper_swagger::{client::remote::RemoteApiClient, config::Config, web};
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::from_env();
    let client = RemoteApiClient::new(cfg.remote_base);
    let app = web::routes(client);

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, cfg.server_port)).await?;
    println!("🚀 http://localhost:{}/swagger-ui", cfg.server_port);
    axum::serve(listener, app).await?;
    Ok(())
}