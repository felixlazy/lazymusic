use std::error::Error;

use tokio::time::{Duration, Interval, MissedTickBehavior, interval};

pub struct App {
    running: bool,
    tui_interval: Interval,
}
impl Default for App {
    fn default() -> Self {
        let mut tui_interval = interval(Duration::from_millis(500));
        tui_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        Self {
            running: Default::default(),
            tui_interval,
        }
    }
}
impl App {
    /// 异步运行主循环
    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // 初始化终端，开启 TUI 环境
        let mut terminal = ratatui::init();

        self.start(); // 设置程序状态为运行中

        // 主循环：程序运行期间不断处理事件和定时器
        while self.running {
            tokio::select! {
                // 定时器触发事件，定时器触发更新一次 UI
                _ = self.tui_interval.tick() => {
                    // 这里绘制 UI（当前例子为空实现）
                    terminal.draw(|_f| ())?;
                }
            }
        }

        // 退出主循环后，恢复终端状态
        ratatui::restore();

        Ok(())
    }

    /// 返回程序是否正在运行
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// 启动程序
    pub fn start(&mut self) {
        self.running = true;
    }

    /// 停止程序
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// 设置或更新 TUI 刷新定时器
    pub fn set_tui_interval(&mut self, duration: Duration) {
        let mut new_interval = interval(duration); // 创建新的定时器
        new_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        // 配置错过 tick 时跳过
        self.tui_interval = new_interval; // 替换原有定时器
    }
}
