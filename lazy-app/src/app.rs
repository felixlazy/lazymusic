//! `App` 模块，定义了应用程序的主要结构和逻辑。

use std::error::Error;

// 从 tokio 中导入时间相关的组件
use tokio::time::{Duration, Interval, MissedTickBehavior, interval};

/// `App` 结构体，代表整个应用程序。
///
/// 它包含了应用程序的状态、事件处理器和 TUI。
pub struct App {
    running: bool,          // 表示应用程序是否正在运行
    tui_interval: Interval, // TUI 刷新定时器
}

impl Default for App {
    /// 创建一个默认的 `App` 实例。
    fn default() -> Self {
        // 创建一个每 100 毫秒触发一次的定时器
        let mut tui_interval = interval(Duration::from_millis(100));
        // 如果错过了 tick，则跳过，以防止 UI 刷新堆积
        tui_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        Self {
            running: Default::default(),
            tui_interval,
        }
    }
}

impl App {
    /// 异步运行主循环。
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error>>` - 如果成功，返回 `Ok(())`，否则返回一个错误。
    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.start(); // 设置程序状态为运行中

        // 主循环：程序运行期间不断处理事件和定时器
        while self.running {
            tokio::select! {
                // 定时器触发事件，定时器触发更新一次 UI
                _ = self.tui_interval.tick() => {
                    // 绘制 TUI
                }
            }
        }

        // 退出主循环后，恢复终端状态
        ratatui::restore();

        Ok(())
    }

    /// 返回程序是否正在运行。
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// 启动程序。
    pub fn start(&mut self) {
        self.running = true;
    }

    /// 停止程序。
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// 设置或更新 TUI 刷新定时器。
    ///
    /// # Arguments
    ///
    /// * `duration` - 新的刷新周期。
    pub fn set_tui_interval(&mut self, duration: Duration) {
        // 创建新的定时器
        let mut new_interval = interval(duration);
        // 配置错过 tick 时跳过
        new_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
        // 替换原有定时器
        self.tui_interval = new_interval;
    }
}
