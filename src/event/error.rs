use std::fmt;
use std::ops::Deref;

use rust_i18n::t;

/// 错误枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// 未知上报类型
    UnknownPostType(String),
    /// 未知子类型
    UnknownSubType(String),
    /// 事件类型转换错误
    UnableConvert {
        /// 源事件类型
        source_event: String,
        /// 目标事件类型
        target_event: String,
        /// 转换失败原因
        because: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = match self {
            Self::UnknownPostType(it) => {
                t!("event.error.Error.UnknownPostType", it => it)
            }
            Self::UnknownSubType(it) => {
                t!("event.error.Error.UnknownSubType", it => it)
            }
            Self::UnableConvert {
                source_event,
                target_event,
                because,
            } => {
                t!("event.error.Error.UnableConvert", source_event => source_event, target_event => target_event, because => because)
            }
        };

        f.write_str(t.deref())
    }
}
