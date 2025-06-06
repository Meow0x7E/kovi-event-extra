use std::ops::Deref;

use kovi::NoticeEvent;
use rust_i18n::t;
use struct_name::StructName;
use struct_name_macro::StructName;

use super::{NoticeType, SubType};
use crate::event::{Error, PostType};
use crate::{_unable_convert, is_none_and_return};

/// 表示戳一戳的通知事件
///
/// 封装了戳一戳事件的特定字段，并提供便捷的访问方法。
///
/// # 类型转换
/// 通过 [`TryFrom`] 实现从 [`NoticeEvent`] 的安全转换，如果转换失败则证明事件不是戳一戳事件：
///
/// ```rust,no_run
/// use std::ops::Deref;
///
/// use kovi::{NoticeEvent, PluginBuilder as plugin, log};
/// use kovi_event_extra::event::notice::PokeNoticeEvent;
///
/// plugin::on_notice(|it| async move {
///     let event = match PokeNoticeEvent::try_from(it.deref()) {
///         Ok(it) => it,
///         Err(it) => {
///             log::trace!("{}", it);
///             return;
///         }
///     };
/// });
/// ```
#[derive(Debug, Clone, StructName)]
pub struct PokeNoticeEvent {
    /// 上报类型。固定为 [`PostType::Notice`]
    pub post_type: PostType,
    /// 通知类型。固定为 [`NoticeType::Notify`]
    pub notice_type: NoticeType,
    /// 提示类型。固定为 [`SubType::Poke`]
    pub sub_type: SubType,
    /// 群号。如果为 None 则为私聊戳一戳
    pub group_id: Option<i64>,
    /// 发送者 QQ 号
    pub user_id: i64,
    /// 被戳者 QQ 号
    pub target_id: i64,

    /// 原始的 [NoticeEvent]
    original_event: NoticeEvent
}

impl PokeNoticeEvent {
    /// 如果戳一戳事件来自群聊则为 [`true`]
    pub fn is_group(&self) -> bool { self.group_id.is_some() }
    /// 如果戳一戳事件来自私聊则为 [`true`]
    pub fn is_private(&self) -> bool { self.group_id.is_none() }
}

impl Deref for PokeNoticeEvent {
    type Target = NoticeEvent;

    /// 获取原始的 [`NoticeEvent`] 引用
    fn deref(&self) -> &Self::Target { &self.original_event }
}

impl TryFrom<NoticeEvent> for PokeNoticeEvent {
    type Error = Error;

    fn try_from(value: NoticeEvent) -> Result<Self, Self::Error> {
        macro_rules! unable_convert {
            ($it:ident) => {
                _unable_convert!(NoticeEvent, $it)
            };
        }

        let json = &value.original_json;

        Ok(Self {
            post_type: {
                let it = PostType::try_from(value.post_type.as_str())?;

                if it != PostType::Notice {
                    let because = t!(r#"global.ne"#, a => "post_type", b => PostType::Notice);
                    unable_convert!(because);
                }

                it
            },
            notice_type: {
                let it = NoticeType::try_from(value.notice_type.as_str())?;

                if it != NoticeType::Notify {
                    let because = t!(r#"global.ne"#, a => "notice_type", b => NoticeType::Notify);
                    unable_convert!(because);
                }

                it
            },
            sub_type: {
                let it = SubType::try_from(is_none_and_return!(
                    json, "sub_type", as_str
                ))?;

                if it != SubType::Poke {
                    let because =
                        t!(r#"global.ne"#, a => "sub_type", b => SubType::Poke);
                    return Err(unable_convert!(because));
                }

                it
            },
            group_id: json.get("group_id").and_then(|it| it.as_i64()),
            user_id: is_none_and_return!(json, "user_id", as_i64),
            target_id: is_none_and_return!(json, "target_id", as_i64),
            original_event: value
        })
    }
}

impl TryFrom<&NoticeEvent> for PokeNoticeEvent {
    type Error = Error;

    fn try_from(value: &NoticeEvent) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}
