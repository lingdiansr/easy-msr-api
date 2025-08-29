use msr_api_rs::{client::remote::RemoteApiClient, web};
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用MSR官方API地址
    let client = RemoteApiClient::new("https://monster-siren.hypergryph.com/api".to_string());
    let app = web::routes_with_swagger(client); // 需要启用swagger-ui feature

    // 启动服务器
    let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 9980)).await?;
    println!("服务器运行在 http://localhost:9980");
    println!("Swagger UI文档: http://localhost:9980/swagger-ui/");
    axum::serve(listener, app).await?;

    Ok(())
}
