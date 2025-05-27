use std::ops::Deref;

use kovi::NoticeEvent;
use rust_i18n::t;
use struct_name::StructName;
use struct_name_macro::StructName;

use super::{NoticeType, SubType};
use crate::event::{Error, PostType};
use crate::{_unable_convert, is_none_and_return};

/// 表示群成员减少的通知事件
///
/// 封装了群成员减少事件的特定字段，并提供便捷的访问方法。
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
pub struct GroupDecreaseNoticeEvent {
    /// 上报类型。固定为 [`PostType::Notice`]
    pub post_type: PostType,
    /// 通知类型。固定为 [`NoticeType::GroupDecrease`]
    pub notice_type: NoticeType,
    /// 事件子类型。可能为 [`SubType::Leave`]、[`SubType::Kick`] 或 [`SubType::KickMe`]
    pub sub_type: SubType,
    /// 群号
    pub group_id: i64,
    /// 操作者 QQ 号。如果是主动退群，则与 [`Self::user_id`] 相同
    pub operator_id: i64,
    /// 离开者 QQ 号
    pub user_id: i64,

    /// 原始的 [NoticeEvent]
    original_event: NoticeEvent
}

impl Deref for GroupDecreaseNoticeEvent {
    type Target = NoticeEvent;

    fn deref(&self) -> &Self::Target { &self.original_event }
}

impl TryFrom<NoticeEvent> for GroupDecreaseNoticeEvent {
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

                if it != NoticeType::GroupDecrease {
                    let because = t!(r#"global.ne"#, a => "notice_type", b => NoticeType::GroupDecrease);
                    return Err(unable_convert!(because));
                }

                it
            },
            sub_type: {
                let it = SubType::try_from(is_none_and_return!(
                    json, "sub_type", as_str
                ))?;

                if it != SubType::Leave
                    && it != SubType::Kick
                    && it != SubType::KickMe
                {
                    let because = t!(r#"event.notice.group_decrease.GroupDecreaseNoticeEvent.SubType"#, a => SubType::Leave, b => SubType::Kick, c => SubType::KickMe);
                    return Err(unable_convert!(because));
                }

                it
            },
            group_id: is_none_and_return!(json, "group_id", as_i64),
            operator_id: is_none_and_return!(json, "operator_id", as_i64),
            user_id: is_none_and_return!(json, "user_id", as_i64),
            original_event: value
        })
    }
}

impl TryFrom<&NoticeEvent> for GroupDecreaseNoticeEvent {
    type Error = Error;

    fn try_from(value: &NoticeEvent) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}
