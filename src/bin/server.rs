use msr_api_rs::{client::remote::RemoteApiClient, config::Config, web};
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::from_env();
    let client = RemoteApiClient::new(cfg.remote_base);
    
    #[cfg(feature = "swagger-ui")]
    {
        let app = web::routes_with_swagger(client);
        let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, cfg.server_port)).await?;
        println!("🚀 Server running at http://localhost:{}", cfg.server_port);
        println!("📚 Swagger UI available at http://localhost:{}/swagger-ui", cfg.server_port);
        axum::serve(listener, app).await?;
    }
    
    #[cfg(not(feature = "swagger-ui"))]
    {
        let app = web::routes(client);
        let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, cfg.server_port)).await?;
        println!("🚀 Server running at http://localhost:{}", cfg.server_port);
        println!("💡 Build with --features swagger-ui to enable Swagger UI");
        axum::serve(listener, app).await?;
    }
    
    Ok(())
}
