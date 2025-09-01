//! # 客户端模块
//! 
//! 提供与MSR API交互的客户端实现。
//! 
//! ## 模块结构
//! 
//! - [`remote`] - 远程API客户端实现，提供完整的API调用功能
//! - `tests` - 单元测试模块（仅在测试模式下可用）

pub mod remote;

#[cfg(test)]
mod tests;
