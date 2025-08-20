pub trait Draw {
    fn draw(&mut self, frame: &mut ratatui::Frame);
}
