use std::ops::Deref;

use kovi::NoticeEvent;
use rust_i18n::t;
use struct_name::StructName;
use struct_name_macro::StructName;

use super::{NoticeType, SubType};
use crate::event::{Error, PostType};
use crate::{_unable_convert, is_none_and_return};

/// 表示群管理员变动的通知事件
///
/// 封装了群管理员变动事件的特定字段，并提供便捷的访问方法。
///
/// # 类型转换
/// 通过 [`TryFrom`] 实现从 [`NoticeEvent`] 的安全转换，如果转换失败则证明事件不是群管理员变动事件：
///
/// ```rust,no_run
/// use std::ops::Deref;
///
/// use kovi::{NoticeEvent, PluginBuilder as plugin, log};
/// use kovi_event_extra::event::notice::GroupAdminNoticeEvent;
///
/// plugin::on_notice(|it| async move {
///     let event = match GroupAdminNoticeEvent::try_from(it.deref()) {
///         Ok(it) => it,
///         Err(it) => {
///             log::trace!("{}", it);
///             return;
///         }
///     };
/// });
/// ```
#[derive(Debug, Clone, StructName)]
pub struct GroupAdminNoticeEvent {
    /// 上报类型。固定为 [`PostType::Notice`]
    pub post_type: PostType,
    /// 通知类型。固定为 [`NoticeType::GroupAdmin`]
    pub notice_type: NoticeType,
    /// 事件子类型。可能为 [`SubType::Set`] 或 [`SubType::Unset`]
    pub sub_type: SubType,
    /// 群号
    pub group_id: i64,
    /// 管理员 QQ 号
    pub user_id: i64,

    /// 原始的 [NoticeEvent]
    original_event: NoticeEvent
}

impl Deref for GroupAdminNoticeEvent {
    type Target = NoticeEvent;

    fn deref(&self) -> &Self::Target { &self.original_event }
}

impl TryFrom<NoticeEvent> for GroupAdminNoticeEvent {
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
                    return Err(unable_convert!(because));
                }

                it
            },
            notice_type: {
                let it = NoticeType::try_from(value.notice_type.as_str())?;

                if it != NoticeType::Notify {
                    let because = t!(r#"global.ne"#, a => "notice_type", b => NoticeType::Notify);
                    return Err(unable_convert!(because));
                }

                it
            },
            sub_type: {
                let it = SubType::try_from(is_none_and_return!(
                    json, "sub_type", as_str
                ))?;

                if it != SubType::Set && it != SubType::Unset {
                    let because = t!(r#"event.notice.group_admin.GroupAdminNoticeEvent.SubType"#, a => SubType::Set, b => SubType::Unset);
                    return Err(unable_convert!(because));
                }

                it
            },
            group_id: is_none_and_return!(json, "group_id", as_i64),
            user_id: is_none_and_return!(json, "user_id", as_i64),
            original_event: value
        })
    }
}

impl TryFrom<&NoticeEvent> for GroupAdminNoticeEvent {
    type Error = Error;

    fn try_from(value: &NoticeEvent) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}
