//! Kovi 的事件处理扩展库
//!
//! 提供对各类事件的增强处理能力
//!
//! ## 功能特性
//! - 类型安全的通知事件解析
//! - 国际化的错误消息支持
//! - 便捷的事件类型判断方法

rust_i18n::i18n!();

pub mod event;
#[macro_use]
pub(crate) mod r#macro;
