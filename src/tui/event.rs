use std::collections::HashMap;

use crate::tui::types::KeyStatus;
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use tokio_stream::StreamExt;
pub struct EventUi {
    events: EventStream,
    keymap: HashMap<KeyCode, KeyStatus>,
}
impl EventUi {
    fn new() -> Self {
        Self {
            events: EventStream::new(),
            keymap: Self::default_keybindings(),
        }
    }
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
    pub async fn read_event_next(&mut self) -> Result<KeyStatus, Box<dyn std::error::Error>> {
        let event = self.events.next().await.expect("no input event received")?;
        self.handle_event(&event)
    }
    pub fn handle_event(&mut self, event: &Event) -> Result<KeyStatus, Box<dyn std::error::Error>> {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                let status = self
                    .keymap
                    .get(&key.code)
                    .copied()
                    .unwrap_or(KeyStatus::NoOp);
                Ok(status)
            } else {
                Err("Key not pressed".into())
            }
        } else {
            Err("Key not pressed".into())
        }
    }
    pub fn add_keybindings(&mut self, key_bindings: HashMap<KeyCode, KeyStatus>) {
        self.keymap.extend(key_bindings);
    }
}

impl Default for EventUi {
    fn default() -> Self {
        Self::new()
    }
}
