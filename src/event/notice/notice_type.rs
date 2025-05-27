use std::fmt;

use crate::event::Error;

/// 通知事件消息类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoticeType {
    /// 群文件上传
    GroupUpload,
    /// 群管理员变动
    GroupAdmin,
    /// 群成员减少
    GroupDecrease,
    /// 群成员增加
    GroupIncrease,
    /// 群禁言
    GroupBan,
    /// 好友添加
    FriendAdd,
    /// 群消息撤回
    GroupRecall,
    /// 好友消息撤回
    FriendRecall,
    /// 群内戳一戳、群红包运气王、群成员荣誉变更
    Notify
}

impl fmt::Display for NoticeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::GroupUpload => "group_upload",
            Self::GroupAdmin => "group_admin",
            Self::GroupDecrease => "group_decrease",
            Self::GroupIncrease => "group_increase",
            Self::GroupBan => "group_ban",
            Self::FriendAdd => "friend_add",
            Self::GroupRecall => "group_recall",
            Self::FriendRecall => "friend_recall",
            Self::Notify => "notify"
        };

        f.write_str(s)
    }
}

impl TryFrom<&str> for NoticeType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "group_upload" => Ok(Self::GroupUpload),
            "group_admin" => Ok(Self::GroupAdmin),
            "group_decrease" => Ok(Self::GroupDecrease),
            "group_increase" => Ok(Self::GroupIncrease),
            "group_ban" => Ok(Self::GroupBan),
            "friend_add" => Ok(Self::FriendAdd),
            "group_recall" => Ok(Self::GroupRecall),
            "friend_recall" => Ok(Self::FriendRecall),
            "notify" => Ok(Self::Notify),
            _ => Err(Self::Error::UnknownNoticeType(value.to_string()))
        }
    }
}
