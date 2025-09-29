use std::collections::HashMap;

use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use tokio_stream::StreamExt;

/// 定义按键状态枚举，用于表示用户操作
#[derive(Clone, Copy)]
pub enum KeyStatus {
    Quit,             // 退出程序
    TogglePlay,       // 播放/暂停切换
    VolumeIncrease,   // 增加音量
    VolumeDecrease,   // 减少音量
    ProgressIncrease, // 快进
    ProgressDecrease, // 快退
    PickerNext,       // 选择下一个项目
    PickerPrev,       // 选择上一个项目
    SwitchMode,       // 切换模式
    NextTrack,        // 下一首
    PrevTrack,        // 上一首
    PlaySelected,     // 播放当前选中的项目
    NoOp,             // 无操作
}

impl Default for KeyStatus {
    fn default() -> Self {
        Self::NoOp // 默认按键状态为无操作
    }
}

/// 事件处理器结构体，用于异步读取终端事件并映射为 KeyStatus
pub struct EventHandler {
    events: EventStream,                 // 异步事件流，用于监听终端事件
    keymap: HashMap<KeyCode, KeyStatus>, // 按键映射表，将 KeyCode 映射为 KeyStatus
}

impl EventHandler {
    /// 构造函数，初始化事件流和默认按键绑定
    fn new() -> Self {
        Self {
            events: EventStream::new(),          // 初始化异步事件流
            keymap: Self::default_keybindings(), // 初始化默认按键映射
        }
    }

    /// 默认按键绑定
    fn default_keybindings() -> HashMap<KeyCode, KeyStatus> {
        use KeyCode::*;
        use KeyStatus::*;

        HashMap::from([
            (Char('q'), Quit),             // q → 退出
            (Char('p'), TogglePlay),       // p → 播放/暂停
            (Char('+'), VolumeIncrease),   // + → 增加音量
            (Char('-'), VolumeDecrease),   // - → 减少音量
            (Char('l'), ProgressIncrease), // l → 快进
            (Char('h'), ProgressDecrease), // h → 快退
            (Char('j'), PickerNext),       // j → 选择下一个
            (Char('k'), PickerPrev),       // k → 选择上一个
            (Char('m'), SwitchMode),       // m → 切换模式
            (Char(']'), NextTrack),        // ] → 下一首
            (Char('['), PrevTrack),        // [ → 上一首
            (Enter, PlaySelected),         // Enter → 播放选中项目
        ])
    }

    /// 异步读取下一个按键事件，并返回对应的 KeyStatus
    pub async fn next_key_status(&mut self) -> Option<KeyStatus> {
        self.events.next().await.and_then(|maybe_result| {
            maybe_result
                // 如果事件流出错，打印错误信息
                .map_err(|e| eprintln!("Event stream error: {:?}", e))
                .ok()
                .map(|event| self.handle_event(&event)) // 将 Event 转换为 KeyStatus
        })
    }

    /// 处理单个事件，将 Event 映射为 KeyStatus
    pub fn handle_event(&mut self, event: &Event) -> KeyStatus {
        if let Event::Key(key) = event {
            // 如果事件是按键事件
            if key.kind == KeyEventKind::Press {
                // 只处理按下事件（忽略释放/重复）
                return self
                    .keymap
                    .get(&key.code) // 查找按键映射表
                    .copied() // 将 &KeyStatus 转为 KeyStatus
                    .unwrap_or(KeyStatus::NoOp); // 未绑定按键返回 NoOp
            }
        }
        KeyStatus::NoOp // 非按键事件返回 NoOp
    }

    /// 添加或扩展自定义按键绑定
    pub fn add_keybindings(&mut self, key_bindings: HashMap<KeyCode, KeyStatus>) {
        self.keymap.extend(key_bindings); // 合并新的按键映射
    }
}

/// 默认实现，调用 new() 构造
impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
