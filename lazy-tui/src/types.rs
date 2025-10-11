use std::{borrow::Cow, time::Duration};

/// TUI 事件枚举
///
/// 用于在 TUI 组件之间传递消息和状态。
#[derive(Clone)]
pub enum TuiEnent<'a> {
    /// 切换播放/暂停状态
    Playback,
    /// 调整音量
    ///
    /// `i8` 表示音量变化的增量或绝对值。
    Volumei(i8),
    /// 更新播放进度
    ///
    /// 第一个 `Duration` 是当前播放时间，第二个是总时长。
    PlaybackProgress(Duration, Duration),
    /// 切换播放模式（如循环、随机等）
    PlaybackMode,
    /// 更新艺术家信息
    Artist(Cow<'a, str>),
    /// 更新曲目信息
    Track(Cow<'a, str>),
}
