use std::collections::HashMap;

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

    /// 添加或扩展自定义按键绑定
    pub fn add_keybindings(&mut self, key_bindings: HashMap<(KeyCode, KeyModifiers), KeyStatus>) {
        self.keymap.extend(key_bindings); // 合并新的按键映射
    }
}

/// 默认实现，调用 new() 构造
impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// 将快捷键字符串解析为 `(KeyCode, KeyModifiers)` 元组。
///
/// 该函数用于将用户自定义的快捷键配置（如 "<c-a>"）转换为 crossterm 能理解的格式。
///
/// # 格式规则
///
/// - **单个字符**: 普通字符直接映射，例如 `q` -> `KeyCode::Char('q')`。
/// - **特殊按键**: 使用尖括号包裹，例如 `<Enter>` -> `KeyCode::Enter`。
/// - **组合键**: 修饰符和按键用 `-` 分隔，例如 `<c-a>` -> `(KeyCode::Char('a'), KeyModifiers::CONTROL)`。
///
/// # 支持的修饰符 (不区分大小写)
///
/// - `c` 或 `ctrl`: Control
/// - `a` 或 `alt`: Alt
/// - `s` 或 `shift`: Shift
///
/// # 支持的特殊按键 (不区分大小写)
///
/// `Enter`, `Tab`, `Backspace`, `Esc`, `Left`, `Right`, `Up`, `Down`,
/// `Home`, `End`, `PageUp`, `PageDown`, `Delete`, `Insert`, `F1` 到 `F12`。
pub fn parse_key_string(keymap: impl AsRef<str>) -> Option<(KeyCode, KeyModifiers)> {
    let s = keymap.as_ref();
    // 检查是否为 <...> 格式的特殊按键
    if s.starts_with('<') && s.ends_with('>') {
        // 提取尖括号内的内容
        let inner = &s[1..s.len() - 1];
        // 如果内容为空或只有空白，则为无效输入
        if inner.trim().is_empty() {
            return None;
        }

        // 特殊处理内容为 "-" 的情况, 例如 "<->"
        if inner == "-" {
            return Some((KeyCode::Char('-'), KeyModifiers::NONE));
        }

        // 使用 rsplit_once 从右向左分割，最后一部分为按键，前面都是修饰符
        // unwrap_or 用于处理没有修饰符的情况 (例如 "<Enter>")
        let (mod_parts, key_part_str) = inner.rsplit_once('-').unwrap_or(("", inner));

        // 如果按键部分为空白，则为无效输入 (例如 "<c- >")
        if key_part_str.trim().is_empty() {
            return None;
        }
        // 将按键部分转为小写以进行不区分大小写的匹配
        let key_part = key_part_str.to_lowercase();

        let mut modifiers = KeyModifiers::NONE;
        // 如果存在修饰符部分
        if !mod_parts.is_empty() {
            // 再次用 '-' 分割以处理多个修饰符 (例如 "c-s")
            for modifier_part in mod_parts.split('-') {
                // 忽略空的修饰符部分 (例如 "c--a" 中的情况)
                if modifier_part.is_empty() {
                    continue;
                }
                // 匹配修饰符，并叠加到 modifiers 位图中
                match modifier_part.to_lowercase().as_str() {
                    "c" | "ctrl" => modifiers |= KeyModifiers::CONTROL,
                    "a" | "alt" => modifiers |= KeyModifiers::ALT,
                    "s" | "shift" => modifiers |= KeyModifiers::SHIFT,
                    _ => return None, // 遇到未知修饰符，返回 None
                }
            }
        }

        // 匹配按键部分
        let key_code = match key_part.as_str() {
            "enter" => KeyCode::Enter,
            "tab" => KeyCode::Tab,
            "backspace" => KeyCode::Backspace,
            "esc" | "escape" => KeyCode::Esc,
            "left" => KeyCode::Left,
            "right" => KeyCode::Right,
            "up" => KeyCode::Up,
            "down" => KeyCode::Down,
            "home" => KeyCode::Home,
            "end" => KeyCode::End,
            "pageup" => KeyCode::PageUp,
            "pagedown" => KeyCode::PageDown,
            "delete" => KeyCode::Delete,
            "insert" => KeyCode::Insert,
            // 匹配 F1-F12
            key if key.starts_with('f') && key.len() > 1 => {
                if let Ok(n) = key[1..].parse::<u8>() {
                    if (1..=12).contains(&n) {
                        KeyCode::F(n)
                    } else {
                        return None; // F 键超出 1-12 范围
                    }
                } else {
                    return None; // F 键格式错误
                }
            }
            // 匹配单个字符的按键, 例如 "a", "b", "-"
            single_char if single_char.len() == 1 => {
                KeyCode::Char(single_char.chars().next().unwrap())
            }
            _ => return None, // 未知按键
        };

        Some((key_code, modifiers))
    // 如果不是 <...> 格式，则检查是否为单个字符
    } else if s.len() == 1 {
        Some((KeyCode::Char(s.chars().next().unwrap()), KeyModifiers::NONE))
    } else {
        // 其他所有格式均为无效
        None
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
    fn test_parse_key_string() {
        // Single characters
        assert_eq!(
            parse_key_string("q"),
            Some((KeyCode::Char('q'), KeyModifiers::NONE))
        );
        assert_eq!(
            parse_key_string("L"),
            Some((KeyCode::Char('L'), KeyModifiers::NONE))
        );
        assert_eq!(
            parse_key_string("+"),
            Some((KeyCode::Char('+'), KeyModifiers::NONE))
        );

        // Special keys
        assert_eq!(
            parse_key_string("<Enter>"),
            Some((KeyCode::Enter, KeyModifiers::NONE))
        );
        assert_eq!(
            parse_key_string("<esc>"),
            Some((KeyCode::Esc, KeyModifiers::NONE))
        );
        assert_eq!(
            parse_key_string("<F5>"),
            Some((KeyCode::F(5), KeyModifiers::NONE))
        );

        // Simple modifiers
        assert_eq!(
            parse_key_string("<c-a>"),
            Some((KeyCode::Char('a'), KeyModifiers::CONTROL))
        );
        assert_eq!(
            parse_key_string("<a-a>"),
            Some((KeyCode::Char('a'), KeyModifiers::ALT))
        );
        assert_eq!(
            parse_key_string("<s-a>"),
            Some((KeyCode::Char('a'), KeyModifiers::SHIFT))
        );

        // Case insensitivity
        assert_eq!(
            parse_key_string("<C-a>"),
            Some((KeyCode::Char('a'), KeyModifiers::CONTROL))
        );
        assert_eq!(
            parse_key_string("<c-A>"),
            Some((KeyCode::Char('a'), KeyModifiers::CONTROL))
        );
        assert_eq!(
            parse_key_string("<CTRL-Enter>"),
            Some((KeyCode::Enter, KeyModifiers::CONTROL))
        );

        // Multiple modifiers
        assert_eq!(
            parse_key_string("<c-s-b>"),
            Some((
                KeyCode::Char('b'),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT
            ))
        );
        assert_eq!(
            parse_key_string("<c-a-s-F11>"),
            Some((
                KeyCode::F(11),
                KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SHIFT
            ))
        );

        // Invalid cases
        assert_eq!(parse_key_string("longstring"), None);
        assert_eq!(parse_key_string("<x-a>"), None); // invalid modifier
        assert_eq!(parse_key_string("<f13>"), None); // invalid f key
        assert_eq!(parse_key_string("<c-enter-a>"), None); // key must be last
        assert_eq!(parse_key_string(""), None);
        assert_eq!(parse_key_string("< >"), None);
        assert_eq!(parse_key_string("<c- >"), None);
        assert_eq!(
            parse_key_string("<->"),
            Some((KeyCode::Char('-'), KeyModifiers::NONE))
        );
    }
}

