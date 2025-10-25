use std::collections::HashMap;

use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use lazy_core::types::KeyStatus;
use tokio_stream::StreamExt;

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
            (Char('L'), NavbarNext),
            (Char('H'), NavbarPrev),
            (Enter, PlaySelected), // Enter → 播放选中项目
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

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    #[test]
    fn test_default_keybindings() {
        let mut event_handler = EventHandler::new();

        // 测试几个默认按键绑定
        let event_q = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('q'),
            KeyModifiers::NONE,
            KeyEventKind::Press,
        ));
        assert_eq!(event_handler.handle_event(&event_q), KeyStatus::Quit);

        let event_p = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('p'),
            KeyModifiers::NONE,
            KeyEventKind::Press,
        ));
        assert_eq!(event_handler.handle_event(&event_p), KeyStatus::TogglePlay);

        let event_enter = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Enter,
            KeyModifiers::NONE,
            KeyEventKind::Press,
        ));
        assert_eq!(
            event_handler.handle_event(&event_enter),
            KeyStatus::PlaySelected
        );
    }

    #[test]
    fn test_handle_event_noop_for_unmapped_key() {
        let mut event_handler = EventHandler::new();
        let event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('z'), // 一个未绑定的按键
            KeyModifiers::NONE,
            KeyEventKind::Press,
        ));
        assert_eq!(event_handler.handle_event(&event), KeyStatus::NoOp);
    }

    #[test]
    fn test_handle_event_noop_for_key_release() {
        let mut event_handler = EventHandler::new();
        let event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('q'), // 一个已绑定的按键
            KeyModifiers::NONE,
            KeyEventKind::Release, // 但这是一个释放事件
        ));
        assert_eq!(event_handler.handle_event(&event), KeyStatus::NoOp);
    }

    #[test]
    fn test_add_keybindings() {
        let mut event_handler = EventHandler::new();
        let mut new_bindings = HashMap::new();
        new_bindings.insert(KeyCode::Char('a'), KeyStatus::NextTrack);
        event_handler.add_keybindings(new_bindings);

        let event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::NONE,
            KeyEventKind::Press,
        ));
        assert_eq!(event_handler.handle_event(&event), KeyStatus::NextTrack);
    }

    #[test]
    fn test_default_trait() {
        let handler: EventHandler = Default::default();
        assert_eq!(handler.keymap, EventHandler::default_keybindings());
    }
}
