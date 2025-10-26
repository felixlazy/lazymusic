use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyModifiers};
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

/// 从 `&Keymaps` 引用转换为 `HashMap`，用于快速查找键位绑定。
///
/// 这个实现遍历 `Keymaps` 中的所有 `KeymapConfig`，
/// 使用 `parse_key_string` 函数解析 `on` 字符串。
/// 如果 `on` 字符串无效，则该条配置将被忽略。
impl From<&Keymaps> for HashMap<(KeyCode, KeyModifiers), KeyStatus> {
    fn from(val: &Keymaps) -> Self {
        val.configs
            .iter()
            .filter_map(|keymap_config| {
                parse_key_string(&keymap_config.on).map(|key| (key, keymap_config.run))
            })
            .collect()
    }
}

/// 从 `HashMap` 转换为 `Keymaps`，用于将程序内部的键位映射转换回可配置的结构。
///
/// 这个实现遍历 `HashMap` 中的所有条目，
/// 使用 `format_key_string` 函数将 `(KeyCode, KeyModifiers)` 键转换回字符串形式的 `on` 字段。
/// `argument` 和 `desc` 字段会被设置为默认值 `None`。
impl From<HashMap<(KeyCode, KeyModifiers), KeyStatus>> for Keymaps {
    fn from(value: HashMap<(KeyCode, KeyModifiers), KeyStatus>) -> Self {
        Self {
            configs: value
                .into_iter()
                .map(|((code, modifiers), status)| KeymapConfig {
                    on: format_key_string(code, modifiers),
                    run: status,
                    ..Default::default()
                })
                .collect(),
        }
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

/// 将 `(KeyCode, KeyModifiers)` 元组格式化为快捷键字符串。
///
/// 这是 `parse_key_string` 的逆向操作。
pub fn format_key_string(code: KeyCode, modifiers: KeyModifiers) -> String {
    // Simple case: single char, no modifiers. This is the only case not wrapped in <...>.
    if modifiers == KeyModifiers::NONE
        && let KeyCode::Char(c) = code
    {
        return c.to_string();
    }

    let mut parts = Vec::new();

    // Add modifiers in a consistent order (c, a, s)
    if modifiers.contains(KeyModifiers::CONTROL) {
        parts.push("c");
    }
    if modifiers.contains(KeyModifiers::ALT) {
        parts.push("a");
    }
    if modifiers.contains(KeyModifiers::SHIFT) {
        parts.push("s");
    }

    let key_part_str = match code {
        // For modified chars, parse_key_string converts them to lowercase.
        KeyCode::Char(c) => c.to_lowercase().to_string(),
        KeyCode::Enter => "enter".to_string(),
        KeyCode::Tab => "tab".to_string(),
        KeyCode::Backspace => "backspace".to_string(),
        KeyCode::Esc => "esc".to_string(),
        KeyCode::Left => "left".to_string(),
        KeyCode::Right => "right".to_string(),
        KeyCode::Up => "up".to_string(),
        KeyCode::Down => "down".to_string(),
        KeyCode::Home => "home".to_string(),
        KeyCode::End => "end".to_string(),
        KeyCode::PageUp => "pageup".to_string(),
        KeyCode::PageDown => "pagedown".to_string(),
        KeyCode::Delete => "delete".to_string(),
        KeyCode::Insert => "insert".to_string(),
        KeyCode::F(n) => format!("f{}", n),
        // This function does not need to be exhaustive for all KeyCodes,
        // only for those that can be parsed by `parse_key_string`.
        _ => "unknown".to_string(),
    };

    parts.push(key_part_str.as_str());
    format!("<{}>", parts.join("-"))
}

#[cfg(test)]
mod test {
    use super::*;
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

    #[test]
    fn test_keymaps_from_conversion() {
        let keymaps = Keymaps {
            configs: vec![
                KeymapConfig {
                    on: "q".to_string(),
                    run: KeyStatus::Quit,
                    ..Default::default()
                },
                KeymapConfig {
                    on: "<c-p>".to_string(),
                    run: KeyStatus::TogglePlay,
                    ..Default::default()
                },
                KeymapConfig {
                    on: "invalid-key".to_string(), // This one should be ignored
                    run: KeyStatus::NoOp,
                    ..Default::default()
                },
            ],
        };

        let hashmap: HashMap<(KeyCode, KeyModifiers), KeyStatus> = (&keymaps).into();

        // Check that only the 2 valid keymaps were converted
        assert_eq!(hashmap.len(), 2);

        // Check if the valid keys are correctly mapped
        assert_eq!(
            hashmap.get(&(KeyCode::Char('q'), KeyModifiers::NONE)),
            Some(&KeyStatus::Quit)
        );
        assert_eq!(
            hashmap.get(&(KeyCode::Char('p'), KeyModifiers::CONTROL)),
            Some(&KeyStatus::TogglePlay)
        );
    }

    #[test]
    fn test_format_key_string() {
        // Single characters
        assert_eq!(
            format_key_string(KeyCode::Char('q'), KeyModifiers::NONE),
            "q"
        );
        assert_eq!(
            format_key_string(KeyCode::Char('A'), KeyModifiers::NONE),
            "A"
        );

        // Special keys
        assert_eq!(
            format_key_string(KeyCode::Enter, KeyModifiers::NONE),
            "<enter>"
        );
        assert_eq!(format_key_string(KeyCode::F(5), KeyModifiers::NONE), "<f5>");

        // Simple modifiers
        assert_eq!(
            format_key_string(KeyCode::Char('a'), KeyModifiers::CONTROL),
            "<c-a>"
        );
        // It should lowercase the character
        assert_eq!(
            format_key_string(KeyCode::Char('A'), KeyModifiers::CONTROL),
            "<c-a>"
        );
        assert_eq!(
            format_key_string(KeyCode::Enter, KeyModifiers::CONTROL),
            "<c-enter>"
        );

        // Multiple modifiers (order should be consistent: c-a-s)
        assert_eq!(
            format_key_string(
                KeyCode::Char('b'),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT
            ),
            "<c-s-b>"
        );
        assert_eq!(
            format_key_string(
                KeyCode::F(11),
                KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SHIFT
            ),
            "<c-a-s-f11>"
        );
    }

    #[test]
    fn test_keymaps_from_hashmap_conversion() {
        let mut hashmap = HashMap::new();
        hashmap.insert((KeyCode::Char('q'), KeyModifiers::NONE), KeyStatus::Quit);
        hashmap.insert(
            (KeyCode::Char('p'), KeyModifiers::CONTROL),
            KeyStatus::TogglePlay,
        );

        let keymaps: Keymaps = hashmap.into();

        assert_eq!(keymaps.configs.len(), 2);

        // We can't rely on the order, so we need to check for existence.
        let config1_found = keymaps
            .configs
            .iter()
            .any(|c| c.on == "q" && c.run == KeyStatus::Quit);
        let config2_found = keymaps
            .configs
            .iter()
            .any(|c| c.on == "<c-p>" && c.run == KeyStatus::TogglePlay);

        assert!(config1_found, "Config for 'q' not found or incorrect");
        assert!(config2_found, "Config for '<c-p>' not found or incorrect");
    }
}
