use std::collections::HashMap;

use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use tokio_stream::StreamExt;

/// 自定义按键状态枚举，用于表示用户在终端按下的不同按键动作
#[derive(Clone, Copy)]
pub enum KeyStatus {
    Quit,             // 退出程序
    TogglePlay,       // 播放/暂停切换
    VolumeIncrease,   // 增加音量
    VolumeDecrease,   // 减少音量
    ProgressIncrease, // 快进
    ProgressDecrease, // 后退
    PickerNext,       // 选择器向下
    PickerPrev,       // 选择器向上
    SwitchMode,       // 切换模式
    NextTrack,        // 下一首
    PrevTrack,        // 上一首
    PlaySelected,     // 播放选中项
    NoOp,             // 无操作，占位用
}

impl Default for KeyStatus {
    fn default() -> Self {
        Self::NoOp
    }
}

/// 输入管理器，用于异步监听终端事件，并将按键映射到自定义 KeyStatus
pub struct InputManager {
    events: EventStream,                 // 异步事件流，用于监听终端输入
    keymap: HashMap<KeyCode, KeyStatus>, // 按键映射表，KeyCode -> KeyStatus
}

impl InputManager {
    /// 创建 InputManager 实例，初始化事件流和默认按键绑定
    fn new() -> Self {
        Self {
            events: EventStream::new(),
            keymap: Self::default_keybindings(),
        }
    }

    /// 默认按键绑定表
    fn default_keybindings() -> HashMap<KeyCode, KeyStatus> {
        use KeyCode::*;
        use KeyStatus::*;

        HashMap::from([
            (Char('q'), Quit),
            (Char('p'), TogglePlay),
            (Char('+'), VolumeIncrease),
            (Char('-'), VolumeDecrease),
            (Char('l'), ProgressIncrease),
            (Char('h'), ProgressDecrease),
            (Char('j'), PickerNext),
            (Char('k'), PickerPrev),
            (Char('m'), SwitchMode),
        ])
    }

    /// 异步读取下一个按键事件，返回对应的 KeyStatus
    /// 如果事件读取失败或不是按下事件，则返回 None
    pub async fn read_event_next(&mut self) -> Option<KeyStatus> {
        self.events
            .next()
            .await
            .and_then(|res| res.ok().map(|e| self.handle_event(&e)))
    }

    /// 处理单个事件，将 Event 转换为 KeyStatus
    /// 如果不是按下事件，返回 KeyStatus::NoOp
    pub fn handle_event(&mut self, event: &Event) -> KeyStatus {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                self.keymap
                    .get(&key.code)
                    .copied()
                    .unwrap_or(KeyStatus::NoOp)
            } else {
                KeyStatus::NoOp
            }
        } else {
            KeyStatus::NoOp
        }
    }

    /// 添加或覆盖按键绑定
    pub fn add_keybindings(&mut self, key_bindings: HashMap<KeyCode, KeyStatus>) {
        self.keymap.extend(key_bindings);
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}
