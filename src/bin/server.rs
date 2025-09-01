use msr_api_rs::{client::remote::RemoteApiClient, config::Config, web};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::from_env()?;
    info!("配置加载成功: {:?}", cfg);

    let client = RemoteApiClient::new(cfg.remote_base.clone());
    info!("API客户端创建成功，基础URL: {}", cfg.remote_base);

    let app = web::routes(client);
    let listener = tokio::net::TcpListener::bind(&cfg.server_addr()).await?;
    info!("🚀 服务器启动成功: http://{}", cfg.server_addr());
    info!("📚 Swagger UI文档: http://{}/swagger-ui", cfg.server_addr());

    axum::serve(listener, app).await?;

    Ok(())
}