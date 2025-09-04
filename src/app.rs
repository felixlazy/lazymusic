use crate::event::{InputManager, KeyStatus};
use crate::ui::root::RootUi;
use crate::ui::traits::{Renderable, TitleControl};
use std::{default::Default, error::Error};
use tokio::time::{Duration, MissedTickBehavior, interval};

#[derive(Default)]
pub struct App {
    ui: RootUi,
    input: InputManager,
    runing: bool,
}
impl App {
    /// ``` no_run
    /// use lazy_music::app::App;
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut app = App::default();
    /// app.run().await?;
    /// Ok(())
    /// }
    /// ```
    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.start();

        let mut interval = interval(Duration::from_millis(500));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        let mut terminal = ratatui::init();

        self.ui.set_title(" lazy music".to_string());

        #[allow(clippy::while_immutable_condition)]
        while self.runing {
            tokio::select! {
                _=interval.tick()=>{
                    terminal
                        .draw(|f| self.ui.render(f))?;
                }
                Some(key_status)=self.input.read_event_next()=>{
                    self.event_handle(key_status)?;
                }
            }
        }
        ratatui::restore();
        Ok(())
    }
    pub fn stop(&mut self) {
        self.runing = false;
    }
    pub fn start(&mut self) {
        self.runing = true;
    }

    fn event_handle(&mut self, key_status: KeyStatus) -> Result<(), Box<dyn std::error::Error>> {
        match key_status {
            KeyStatus::Quit => self.stop(),
            KeyStatus::TogglePlay => (),
            KeyStatus::VolumeIncrease => (),
            KeyStatus::VolumeDecrease => (),
            KeyStatus::ProgressIncrease => (),
            KeyStatus::ProgressDecrease => (),
            KeyStatus::PickerNext => (),
            KeyStatus::PickerPrev => (),
            KeyStatus::SwitchMode => (),
            KeyStatus::NextTrack => (),
            KeyStatus::PrevTrack => (),
            KeyStatus::PlaySelected => (),
            KeyStatus::NoOp => (),
        }
        Ok(())
    }
}
