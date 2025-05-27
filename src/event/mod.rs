//! 事件处理核心模块
//!
//! 定义事件处理的基础结构和错误类型

mod error;
pub mod notice;
mod post_type;

// 重新导出
pub use error::Error;
pub use post_type::PostType;
