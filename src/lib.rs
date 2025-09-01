//! # MSR API Rust 封装库
//! 
//! 这是一个为MSR API（明日方舟音乐API）提供Rust封装的库，支持直接API调用和Swagger-UI文档。
//! 
//! ## 功能特性
//! 
//! - **核心API封装**：完整的MSR API功能封装，可直接调用
//! - **可选Web路由**：提供Axum Web路由集成，包含Swagger UI文档界面
//! - **类型安全**：完整的DTO类型定义
//! 
//! ## 使用方式
//! 
//! ### 1. 使用默认客户端(推荐)
//! 
//! ```rust
//! use msr_api_rs::MSRApiClient;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 使用默认的MSR API地址
//!     let client = MSRApiClient::new();
//!     
//!     let albums = client.get_all_albums().await?;
//!     println!("专辑数量: {}", albums.data.len());
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ### 2. 作为库直接调用API
//! 
//! ```rust
//! use msr_api_rs::client::remote::RemoteApiClient;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 创建API客户端
//!     let client = RemoteApiClient::new("https://monster-siren.hypergryph.com/api".to_string());
//!     
//!     // 直接调用API方法
//!     let song = client.get_song("123456".to_string()).await?;
//!     println!("歌曲名称: {}", song.data.name);
//!     
//!     Ok(())
//! }
//! ```
//! ### 3. 作为Web服务使用（需要启用web feature）
//! 
//! ```rust
//! use msr_api_rs::{client::remote::RemoteApiClient, web};
//! use std::net::Ipv4Addr;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = RemoteApiClient::new("https://monster-siren.hypergryph.com/api".to_string());
//!     let app = web::routes(client);
//!     
//!     // 启动服务器
//!     let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 8080)).await?;
//!     println!("服务器运行在 http://localhost:8080");
//!     println!("Swagger UI文档: http://localhost:8080/swagger-ui/");
//!     axum::serve(listener, app).await?;
//!     
//!     Ok(())
//! }
//! ```
//! 或者直接在本项目下执行`cargo run --bin server --features web`
//! 
//! ## Cargo Features
//! 
//! - **default**: 无额外功能，仅包含核心API封装
//! - **web**: 启用Web路由和Swagger UI界面支持
//! 
//! ## 模块结构
//! 
//! - [`client`] - API客户端实现
//! - [`config`] - 配置管理
//! - [`error`] - 错误处理
//! - Web路由层（需要启用 `web` feature）
#![cfg_attr(feature = "web", doc = "- [`web`] - Web 路由层")]

pub mod client;
pub mod config;
pub mod error;
pub mod dto;

#[cfg(feature = "web")]
pub mod web;

use crate::{client::remote::RemoteApiClient, dto::*, error::AppError};

/// 默认的MSR API客户端，使用官方API地址
/// 
/// 这个客户端提供了对MSR API的完整封装，使用官方的API地址作为默认配置。
/// 支持所有主要的API调用，包括歌曲、专辑、新闻和搜索功能。
pub struct MSRApiClient {
    inner: RemoteApiClient,
}

impl MSRApiClient {
    /// 创建默认的MSR API客户端，使用官方API地址
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use msr_api_rs::MSRApiClient;
    /// 
    /// let client = MSRApiClient::new();
    /// ```
    pub fn new() -> Self {
        Self {
            inner: RemoteApiClient::new("https://monster-siren.hypergryph.com/api".to_string()),
        }
    }

    /// 使用自定义API地址创建客户端
    /// 
    /// # 参数
    /// 
    /// * `base` - API的基础URL地址
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use msr_api_rs::MSRApiClient;
    /// 
    /// let client = MSRApiClient::with_base("https://custom-api.com".to_string());
    /// ```
    pub fn with_base(base: String) -> Self {
        Self {
            inner: RemoteApiClient::new(base),
        }
    }

    /// 获取指定ID的歌曲详情
    pub async fn get_song(&self, id: String) -> Result<SongResp, AppError> {
        self.inner.get_song(id).await
    }

    /// 获取所有歌曲列表
    pub async fn get_all_songs(&self) -> Result<AllSongsResp, AppError> {
        self.inner.get_all_songs().await
    }

    /// 获取指定ID的专辑信息
    pub async fn get_album(&self, id: String) -> Result<AlbumResp, AppError> {
        self.inner.get_album(id).await
    }

    /// 获取指定ID的专辑详情（包含歌曲列表）
    pub async fn get_album_detail(&self, id: String) -> Result<AlbumDetailResp, AppError> {
        self.inner.get_album_detail(id).await
    }

    /// 获取所有专辑列表
    pub async fn get_all_albums(&self) -> Result<ApiResp<Vec<AllAlbumsItem>>, AppError> {
        self.inner.get_all_albums().await
    }

    /// 获取所有新闻列表
    /// 
    /// # 参数
    /// 
    /// * `last_cid` - 可选参数，用于分页，从指定cid之后开始获取
    pub async fn get_all_news(&self, last_cid: Option<String>) -> Result<SearchNewsResp, AppError> {
        self.inner.get_all_news(last_cid).await
    }

    /// 获取指定ID的新闻详情
    pub async fn get_news_detail(&self, id: String) -> Result<NewsDetailResp, AppError> {
        self.inner.get_news_detail(id).await
    }

    /// 获取字体配置信息
    pub async fn get_font(&self) -> Result<FontResp, AppError> {
        self.inner.get_font().await
    }

    /// 综合搜索（同时搜索专辑和新闻）
    /// 
    /// # 参数
    /// 
    /// * `keyword` - 搜索关键词
    pub async fn search(&self, keyword: String) -> Result<SearchResp, AppError> {
        self.inner.search(keyword).await
    }

    /// 搜索专辑
    /// 
    /// # 参数
    /// 
    /// * `keyword` - 搜索关键词
    /// * `last_cid` - 可选参数，用于分页
    pub async fn search_albums(
        &self,
        keyword: String,
        last_cid: Option<String>,
    ) -> Result<SearchAlbumResp, AppError> {
        self.inner.search_albums(keyword, last_cid).await
    }

    /// 搜索新闻
    /// 
    /// # 参数
    /// 
    /// * `keyword` - 搜索关键词
    /// * `last_cid` - 可选参数，用于分页
    pub async fn search_news(
        &self,
        keyword: String,
        last_cid: Option<String>,
    ) -> Result<SearchNewsResp, AppError> {
        self.inner.search_news(keyword, last_cid).await
    }
}
