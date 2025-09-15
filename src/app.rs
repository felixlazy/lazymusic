use tokio::time::{self, Duration, Interval, MissedTickBehavior, interval};

use crate::event::{EventHandler, KeyStatus};
use std::error::Error;

/// 主应用结构体，用于管理 TUI 应用的运行状态、按键事件和刷新定时器
pub struct App {
    running: bool,          // 程序是否正在运行
    event: EventHandler,    // 异步事件处理器，用于监听按键事件
    tui_interval: Interval, // TUI 刷新定时器，每隔一段时间触发一次
}

impl Default for App {
    fn default() -> Self {
        // 创建 TUI 定时器，默认 500ms 刷新一次
        let mut tui_interval = time::interval(Duration::from_millis(500));
        // 配置定时器，错过 tick 时跳过，不累积等待
        tui_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        Self {
            running: Default::default(), // 默认未运行
            event: Default::default(),   // 默认事件处理器
            tui_interval,                // 默认刷新间隔 500ms
        }
    }
}

impl App {
    /// 异步运行主循环
    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // 初始化终端，开启 TUI 环境
        let mut terminal = ratatui::init();

        // 配置定时器，错过 tick 时跳过，不累积等待
        self.tui_interval
            .set_missed_tick_behavior(MissedTickBehavior::Skip);

        self.start(); // 设置程序状态为运行中

        // 主循环：程序运行期间不断处理事件和定时器
        while self.running {
            tokio::select! {
                // 异步等待按键事件
                key_status = self.event.next_key_status() => {
                    if let Some(key) = key_status {
                        // 如果有按键事件，调用事件处理器
                        self.event_handler(key);
                    }
                }
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

    /// 处理按键事件，将 KeyStatus 映射为具体操作
    fn event_handler(&mut self, key_status: KeyStatus) {
        match key_status {
            KeyStatus::Quit => self.stop(),    // q → 退出程序
            KeyStatus::TogglePlay => (),       // p → 播放/暂停
            KeyStatus::VolumeIncrease => (),   // + → 增加音量
            KeyStatus::VolumeDecrease => (),   // - → 减少音量
            KeyStatus::ProgressIncrease => (), // l → 快进
            KeyStatus::ProgressDecrease => (), // h → 快退
            KeyStatus::PickerNext => (),       // j → 选择下一个
            KeyStatus::PickerPrev => (),       // k → 选择上一个
            KeyStatus::SwitchMode => (),       // m → 切换模式
            KeyStatus::NextTrack => (),        // ] → 下一首
            KeyStatus::PrevTrack => (),        // [ → 上一首
            KeyStatus::PlaySelected => (),     // Enter → 播放选中
            KeyStatus::NoOp => (),             // 无操作
        }
    }
}
