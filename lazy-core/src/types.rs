use serde::{Deserialize, Serialize};
/// 定义按键状态枚举，用于表示用户操作
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum KeyStatus {
    #[serde(rename = "quit")]
    Quit, // 退出程序
    #[serde(rename = "toggle play")]
    TogglePlay, // 播放/暂停切换
    #[serde(rename = "volume increase")]
    VolumeIncrease, // 增加音量
    #[serde(rename = "volume decrease")]
    VolumeDecrease, // 减少音量
    #[serde(rename = "progress increase")]
    ProgressIncrease, // 快进
    #[serde(rename = "progress decrease")]
    ProgressDecrease, // 快退
    #[serde(rename = "picker next")]
    PickerNext, // 选择下一个项目
    #[serde(rename = "picker prev")]
    PickerPrev, // 选择上一个项目
    #[serde(rename = "switch mode")]
    SwitchMode, // 切换模式
    #[serde(rename = "next track")]
    NextTrack, // 下一首
    #[serde(rename = "prev track")]
    PrevTrack, // 上一首
    #[serde(rename = "play selected")]
    PlaySelected, // 播放当前选中的项目
    #[serde(rename = "navbar next")]
    NavbarNext, // 导航栏下一个
    #[serde(rename = "navbar prev")]
    NavbarPrev, // 导航栏上一个
    #[default]
    NoOp, // 无操作
}
