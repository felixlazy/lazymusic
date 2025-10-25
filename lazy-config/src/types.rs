use lazy_core::types::KeyStatus;
use serde::{Deserialize, Serialize};

// Keymaps.toml 示例
//
// [[keymaps]]
// on = "j"
// run = "next track"
// desc = "下一首"
//
// [[keymaps]]
// on = "-"
// run = "volume decrease"
// argument = 10
// desc = "音量减 10"

/// 代表从 TOML 文件中读取的键位映射集合。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Keymaps {
    /// 包含多个键位映射配置的向量。
    #[serde(rename = "keymaps")] // 保持 TOML 中的 [[keymaps]] 不变
    pub configs: Vec<KeymapConfig>,
}

/// 代表单个键位映射的配置。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KeymapConfig {
    /// 触发操作的按键（例如："j"、"k"、"enter"）。
    pub on: String,
    /// 按下按键时要执行的操作。
    pub run: KeyStatus,
    /// 为 'run' 命令提供额外的参数。
    /// Благодаря `#[serde(untagged)]` on `ActionArgument`,
    /// 你可以直接在 TOML 中写值，例如：
    /// `argument = 10`  (对应 Value(10))
    /// `argument = true` (对应 Enable(true))
    pub argument: Option<ActionArgument>,
    /// 对键位映射功能的可选描述。
    pub desc: Option<String>,
}

/// 为 'run' 命令提供的一个额外参数。
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ActionArgument {
    Value(u8),
    Enable(bool),
}
