use tokio::time::{Duration, interval};

use crate::tui::{Tui, traits::Draw};
pub struct App {
    tui: Tui,
    runing: bool,
}
impl Default for App {
    fn default() -> Self {
        Self {
            tui: Default::default(),
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
    ///     match app.run().await {
    ///         Ok(_) => println!("app run "),
    ///         Err(e) => println!("{e}"),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut draw_interval = interval(Duration::from_millis(500));
        let mut terminal = ratatui::init();
        while self.runing {
            tokio::select! {
                _=draw_interval.tick()=>{
                    terminal.draw(|frame| self.tui.draw(frame))?;
                }
            }
        }
        Ok(())
    }
    pub fn app_exit(&mut self) {
        self.runing = false;
    }
}
