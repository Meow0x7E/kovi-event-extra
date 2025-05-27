use std::fmt;

use crate::event::Error;

/// 通知事件子类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubType {
    /// 设置管理员
    Set,
    /// 取消管理员
    Unset,
    /// 主动退群
    Leave,
    /// 被踢出
    Kick,
    /// Bot 被踢出
    KickMe,
    /// 管理员同意入群
    Approve,
    /// 管理员邀请入群
    Invite,
    /// 禁言
    Ban,
    /// 解除禁言
    LiftBan,
    /// 戳一戳
    Poke,
    /// 群红包运气王
    LuckyKing,
    /// 群成员荣耀变更
    Honor
}

impl fmt::Display for SubType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Set => "set",
            Self::Unset => "unset",
            Self::Leave => "leave",
            Self::Kick => "kick",
            Self::KickMe => "kick_me",
            Self::Approve => "approve",
            Self::Invite => "invite",
            Self::Ban => "ban",
            Self::LiftBan => "lift_ban",
            Self::Poke => "poke",
            Self::LuckyKing => "lucky_king",
            Self::Honor => "honor"
        };

        f.write_str(s)
    }
}

impl TryFrom<&str> for SubType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "set" => Ok(Self::Set),
            "unset" => Ok(Self::Unset),
            "leave" => Ok(Self::Leave),
            "kick" => Ok(Self::Kick),
            "kick_me" => Ok(Self::KickMe),
            "approve" => Ok(Self::Approve),
            "invite" => Ok(Self::Invite),
            "ban" => Ok(Self::Ban),
            "lift_ban" => Ok(Self::LiftBan),
            "poke" => Ok(Self::Poke),
            "lucky_king" => Ok(Self::LuckyKing),
            "honor" => Ok(Self::Honor),
            _ => Err(Self::Error::UnknownSubType(value.to_string()))
        }
    }
}
