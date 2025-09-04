use crate::ui::root::RootUi;
use crate::ui::traits::{Renderable, TitleControl};
use std::{default::Default, error::Error};
use tokio::time::{Duration, MissedTickBehavior, interval};

#[derive(Default)]
pub struct App {
    ui: RootUi,
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
}
