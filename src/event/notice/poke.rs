use std::ops::Deref;

use kovi::NoticeEvent;
use rust_i18n::t;

use crate::event;

/// 表示戳一戳 (Poke) 类型的通知事件
///
/// 封装了戳一戳事件的特定字段，并提供便捷的访问方法。
///
/// # 用法
/// ```rust,no_run
/// use std::ops::Deref;
/// use kovi::{NoticeEvent, PluginBuilder as plugin, log};
/// use kovi_event_extra::event::notice::poke::PokeNoticeEvent;
///
/// plugin::on_notice(|it| {
///     async move {
///         let event = match PokeNoticeEvent::try_from(it.deref()) {
///             Ok(it) => it,
///             Err(it) => {
///                 log::trace!("{}", it);
///                 return;
///             }
///         };
///     }
/// });
/// ```
#[derive(Debug, Clone)]
pub struct PokeNoticeEvent {
    /// 提示类型。固定为 `poke`
    pub sub_type: String,
    /// 群号。如果为 None 则为私聊戳一戳
    pub group_id: Option<i64>,
    /// 发送者 QQ 号
    pub user_id: i64,
    /// 被戳者 QQ 号
    pub target_id: i64,

    /// 原始的 [NoticeEvent]
    original_event: NoticeEvent,
}

impl PokeNoticeEvent {
    /// 如果戳一戳事件来自群聊则为 [`true`]
    pub fn is_group(&self) -> bool {
        self.group_id.is_some()
    }
    /// 如果戳一戳事件来自私聊则为 [`true`]
    pub fn is_private(&self) -> bool {
        self.group_id.is_none()
    }
}

impl Deref for PokeNoticeEvent {
    type Target = NoticeEvent;

    /// 获取原始的 [`NoticeEvent`] 引用
    fn deref(&self) -> &Self::Target {
        &self.original_event
    }
}

impl TryFrom<NoticeEvent> for PokeNoticeEvent {
    type Error = event::ErrorKind;

    fn try_from(value: NoticeEvent) -> Result<Self, Self::Error> {
        let sub_type = {
            let it = value
                .original_json
                .get("sub_type")
                .and_then(|it| it.as_str());
            if it.is_none() {
                #[rustfmt::skip]
                let because = t!(r#"event.notice.poke."impl TryFrom<NoticeEvent> for PokeNoticeEvent".Error.because.sub_type.is_none"#);
                return Err(event::ErrorKind::UnableConvert {
                    source_event: String::from("NoticeEvent"),
                    target_event: String::from("PokeNoticeEvent"),
                    because: because.to_string(),
                });
            }

            let it = it.unwrap();
            if it != "poke" {
                #[rustfmt::skip]
                let because = t!(r#"event.notice.poke."impl TryFrom<NoticeEvent> for PokeNoticeEvent".Error.because.sub_type.ne_poke"#);
                return Err(event::ErrorKind::UnableConvert {
                    source_event: String::from("NoticeEvent"),
                    target_event: String::from("PokeNoticeEvent"),
                    because: because.to_string(),
                });
            }

            it.to_string()
        };
        let group_id = value
            .original_json
            .get("group_id")
            .and_then(|it| it.as_i64());
        let user_id = {
            let it = value
                .original_json
                .get("user_id")
                .and_then(|it| it.as_i64());
            if it.is_none() {
                #[rustfmt::skip]
                let because = t!(r#"event.notice.poke."impl TryFrom<NoticeEvent> for PokeNoticeEvent".Error.because.user_id.is_none"#);
                return Err(event::ErrorKind::UnableConvert {
                    source_event: String::from("NoticeEvent"),
                    target_event: String::from("PokeNoticeEvent"),
                    because: because.to_string(),
                });
            }

            it.unwrap()
        };
        let target_id = {
            let it = value
                .original_json
                .get("target_id")
                .and_then(|it| it.as_i64());
            if it.is_none() {
                #[rustfmt::skip]
                let because = t!(r#"event.notice.poke."impl TryFrom<NoticeEvent> for PokeNoticeEvent".Error.because.user_id.is_none"#);
                return Err(event::ErrorKind::UnableConvert {
                    source_event: String::from("NoticeEvent"),
                    target_event: String::from("PokeNoticeEvent"),
                    because: because.to_string(),
                });
            }

            it.unwrap()
        };

        Ok(Self {
            sub_type,
            group_id,
            user_id,
            target_id,
            original_event: value,
        })
    }
}

impl TryFrom<&NoticeEvent> for PokeNoticeEvent {
    type Error = event::ErrorKind;

    fn try_from(value: &NoticeEvent) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}
