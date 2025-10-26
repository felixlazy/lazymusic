use serde::{Deserialize, Serialize};
/// 定义按键状态枚举，用于表示用户操作
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    // 为了测试自定义的字符串解析，我们用 serde_json 会更简单
    // 因为我们只关心字符串本身如何被反序列化，不关心 TOML 的结构

    #[test]
    fn test_deserialize_key_status_simple() {
        let status: KeyStatus = serde_json::from_str("\"toggle play\"").unwrap();
        assert_eq!(status, KeyStatus::TogglePlay);

        let status: KeyStatus = serde_json::from_str("\"quit\"").unwrap();
        assert_eq!(status, KeyStatus::Quit);
    }

    #[test]
    fn test_deserialize_key_status_with_arg() {
        let status: KeyStatus = serde_json::from_str("\"volume decrease\"").unwrap();
        assert_eq!(status, KeyStatus::VolumeDecrease);
    }

    #[test]
    fn test_deserialize_key_status_with_extra_whitespace() {
        // 测试输入字符串前后和中间的多余空格是否能被正确处理
        let status: KeyStatus = serde_json::from_str("\"volume decrease\"").unwrap();
        assert_eq!(status, KeyStatus::VolumeDecrease);
    }

    #[test]
    fn test_deserialize_key_status_invalid_command() {
        // 测试一个完全不存在的命令
        let result: Result<KeyStatus, _> = serde_json::from_str("\"invalid command\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_key_status_invalid_arg() {
        // 测试参数类型不正确的命令
        let result: Result<KeyStatus, _> = serde_json::from_str("\"volume decrease abc\"");
        assert!(result.is_err());
    }
}
