use tokio::time::{Duration, interval};

use crate::tui::{Tui, event::EventUi, traits::Draw, types::KeyStatus};

pub struct App {
    tui: Tui,
    event: EventUi,
    runing: bool,
}
impl Default for App {
    fn default() -> Self {
        Self {
            tui: Default::default(),
            event: Default::default(),
            runing: true,
        }
    }
}
impl App {
    /// ``` no_run
    /// use lazy_music::app::App;
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut app = App::default();
    ///     app.run().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut draw_interval = interval(Duration::from_millis(500));
        draw_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut terminal = ratatui::init();
        while self.runing {
            tokio::select! {
                _=draw_interval.tick()=>{
                    terminal.draw(|frame| self.tui.draw(frame))?;
                }
                key_status=self.event.read_event_next()=>{
                    if let Ok(key)=key_status{
                        self.event_handle(key)?;
                    }
                }
            }
        }
        ratatui::restore();
        Ok(())
    }
    pub fn app_exit(&mut self) {
        self.runing = false;
    }
    fn event_handle(&mut self, key_status: KeyStatus) -> Result<(), Box<dyn std::error::Error>> {
        match key_status {
            KeyStatus::Quit => self.app_exit(),
            KeyStatus::TogglePlay => todo!(),
            KeyStatus::VolumeIncrease => todo!(),
            KeyStatus::VolumeDecrease => todo!(),
            KeyStatus::ProgressIncrease => todo!(),
            KeyStatus::ProgressDecrease => todo!(),
            KeyStatus::PickerNext => todo!(),
            KeyStatus::PickerPrev => todo!(),
            KeyStatus::SwitchMode => todo!(),
            KeyStatus::NextTrack => todo!(),
            KeyStatus::PrevTrack => todo!(),
            KeyStatus::PlaySelected => todo!(),
            KeyStatus::NoOp => todo!(),
        }
        Ok(())
    }
}
