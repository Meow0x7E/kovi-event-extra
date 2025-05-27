use std::fmt::{self, Display};

use crate::event::Error;

/// 上报类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostType {
    /// 消息事件
    Message,
    /// OneBot 元事件
    MetaEvent,
    /// 通知事件
    Notice,
    /// 请求事件
    Request
}

impl Display for PostType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Message => "message",
            Self::MetaEvent => "meta_event",
            Self::Notice => "notice",
            Self::Request => "request"
        };

        f.write_str(s)
    }
}

impl TryFrom<&str> for PostType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "message" => Ok(Self::Message),
            "meta_event" => Ok(Self::MetaEvent),
            "notice" => Ok(Self::Notice),
            "request" => Ok(Self::Request),
            _ => Err(Self::Error::UnknownSubType(value.to_string()))
        }
    }
}
