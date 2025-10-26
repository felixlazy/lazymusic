use std::collections::{HashMap, HashSet};

use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind, KeyModifiers};
use lazy_core::types::KeyStatus;
use tokio_stream::StreamExt;

/// 事件处理器结构体，用于异步读取终端事件并映射为 KeyStatus
pub struct EventHandler {
    events: Option<EventStream>, // 异步事件流，用于监听终端事件
    keymap: HashMap<(KeyCode, KeyModifiers), KeyStatus>, // 按键映射表，将 (KeyCode, KeyModifiers) 映射为 KeyStatus
}

impl EventHandler {
    /// 构造函数，初始化事件流和默认按键绑定
    fn new() -> Self {
        Self {
            events: Some(EventStream::new()),    // 初始化异步事件流
            keymap: Self::default_keybindings(), // 初始化默认按键映射
        }
    }

    /// 默认按键绑定
    fn default_keybindings() -> HashMap<(KeyCode, KeyModifiers), KeyStatus> {
        use KeyCode::*;
        use KeyStatus::*;

        HashMap::from([
            ((Char('q'), KeyModifiers::NONE), Quit),           // q → 退出
            ((Char('p'), KeyModifiers::NONE), TogglePlay),     // p → 播放/暂停
            ((Char('+'), KeyModifiers::NONE), VolumeIncrease), // + → 增加音量
            ((Char('-'), KeyModifiers::NONE), VolumeDecrease), // - → 减少音量
            ((Char('l'), KeyModifiers::NONE), ProgressIncrease), // l → 快进
            ((Char('h'), KeyModifiers::NONE), ProgressDecrease), // h → 快退
            ((Char('j'), KeyModifiers::NONE), PickerNext),     // j → 选择下一个
            ((Char('k'), KeyModifiers::NONE), PickerPrev),     // k → 选择上一个
            ((Char('m'), KeyModifiers::NONE), SwitchMode),     // m → 切换模式
            ((Char(']'), KeyModifiers::NONE), NextTrack),      // ] → 下一首
            ((Char('['), KeyModifiers::NONE), PrevTrack),      // [ → 上一首
            ((Char('L'), KeyModifiers::NONE), NavbarNext),
            ((Char('H'), KeyModifiers::NONE), NavbarPrev),
            ((Enter, KeyModifiers::NONE), PlaySelected), // Enter → 播放选中项目
        ])
    }

    /// 异步读取下一个按键事件，并返回对应的 KeyStatus
    pub async fn next_key_status(&mut self) -> Option<KeyStatus> {
        if let Some(events) = self.events.as_mut() {
            events.next().await.and_then(|maybe_result| {
                maybe_result
                    // 如果事件流出错，打印错误信息
                    .map_err(|e| eprintln!("Event stream error: {:?}", e))
                    .ok()
                    .map(|event| self.handle_event(&event)) // 将 Event 转换为 KeyStatus
            })
        } else {
            None
        }
    }

    /// 处理单个事件，将 Event 映射为 KeyStatus
    pub fn handle_event(&mut self, event: &Event) -> KeyStatus {
        if let Event::Key(key) = event {
            // 如果事件是按键事件
            if key.kind == KeyEventKind::Press {
                // 只处理按下事件（忽略释放/重复）
                return self
                    .keymap
                    .get(&(key.code, key.modifiers)) // 查找按键映射表
                    .copied() // 将 &KeyStatus 转为 KeyStatus
                    .unwrap_or(KeyStatus::NoOp); // 未绑定按键返回 NoOp
            }
        }
        KeyStatus::NoOp // 非按键事件返回 NoOp
    }

    /// 添加或扩展自定义按键绑定。
    ///
    /// 此方法会先移除 `self.keymap` 中任何与 `key_bindings` 中的值（`KeyStatus`）
    /// 相同的旧绑定，然后再将新的绑定添加进去。
    /// 这确保了每个 `KeyStatus` 只会由最新的配置来触发。
    pub fn add_keybindings(&mut self, key_bindings: HashMap<(KeyCode, KeyModifiers), KeyStatus>) {
        // 1. 收集新绑定中所有出现过的 `KeyStatus`。
        let values_to_replace: HashSet<KeyStatus> = key_bindings.values().copied().collect();

        // 2. 从现有 keymap 中移除所有与新值冲突的旧绑定。
        self.keymap
            .retain(|_key, value| !values_to_replace.contains(value));

        // 3. 添加新的按键绑定。
        self.keymap.extend(key_bindings);
    }

    pub fn read_keybindings(&self) -> HashMap<(KeyCode, KeyModifiers), KeyStatus> {
        self.keymap.clone()
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
        let mut event_handler = EventHandler {
            events: None,
            keymap: EventHandler::default_keybindings(),
        };

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
        let mut event_handler = EventHandler {
            events: None,
            keymap: EventHandler::default_keybindings(),
        };
        let event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('z'), // 一个未绑定的按键
            KeyModifiers::NONE,
            KeyEventKind::Press,
        ));
        assert_eq!(event_handler.handle_event(&event), KeyStatus::NoOp);
    }

    #[test]
    fn test_handle_event_noop_for_key_release() {
        let mut event_handler = EventHandler {
            events: None,
            keymap: EventHandler::default_keybindings(),
        };
        let event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('q'), // 一个已绑定的按键
            KeyModifiers::NONE,
            KeyEventKind::Release, // 但这是一个释放事件
        ));
        assert_eq!(event_handler.handle_event(&event), KeyStatus::NoOp);
    }

    #[test]
    fn test_add_keybindings() {
        let mut event_handler = EventHandler {
            events: None,
            keymap: EventHandler::default_keybindings(),
        };
        let mut new_bindings = HashMap::new();
        new_bindings.insert(
            (KeyCode::Char('a'), KeyModifiers::NONE),
            KeyStatus::NextTrack,
        );
        event_handler.add_keybindings(new_bindings);

        let event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::NONE,
            KeyEventKind::Press,
        ));
        assert_eq!(event_handler.handle_event(&event), KeyStatus::NextTrack);
    }

    #[test]
    fn test_ctrl_keybinding() {
        let mut event_handler = EventHandler {
            events: None,
            keymap: EventHandler::default_keybindings(),
        };
        let mut new_bindings = HashMap::new();
        new_bindings.insert((KeyCode::Char('a'), KeyModifiers::CONTROL), KeyStatus::Quit);
        event_handler.add_keybindings(new_bindings);

        let event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::CONTROL,
            KeyEventKind::Press,
        ));
        assert_eq!(event_handler.handle_event(&event), KeyStatus::Quit);
    }

    #[test]
    fn test_add_keybindings_replaces_by_value() {
        let mut event_handler = EventHandler {
            events: None,
            keymap: EventHandler::default_keybindings(), // Contains (q, Quit)
        };

        // Create a new binding where a different key maps to Quit
        let mut new_bindings = HashMap::new();
        new_bindings.insert((KeyCode::Char('x'), KeyModifiers::CONTROL), KeyStatus::Quit);
        event_handler.add_keybindings(new_bindings);

        // 1. The old key 'q' should no longer map to Quit. It should be NoOp.
        let event_q = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('q'),
            KeyModifiers::NONE,
            KeyEventKind::Press,
        ));
        assert_eq!(event_handler.handle_event(&event_q), KeyStatus::NoOp);

        // 2. The new key '<c-x>' should now map to Quit.
        let event_cx = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('x'),
            KeyModifiers::CONTROL,
            KeyEventKind::Press,
        ));
        assert_eq!(event_handler.handle_event(&event_cx), KeyStatus::Quit);

        // 3. Check total size to be sure
        // Default has 14 items. We removed one and added one. So still 14.
        assert_eq!(event_handler.keymap.len(), 14);
    }
}
