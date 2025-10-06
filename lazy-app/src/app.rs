//! `App` 模块，定义了应用程序的主要结构和逻辑。

use std::error::Error;

// 从 lazy_tui 中导入根 TUI 组件和 RenderTui trait
use lazy_tui::{root::RootTui, traits::RenderTui};
// 从 tokio 中导入时间相关的组件
use tokio::time::{interval, Duration, Interval, MissedTickBehavior};

// 从当前 crate 的 event 模块中导入事件处理器和按键状态
use crate::event::{EventHandler, KeyStatus};

/// `App` 结构体，代表整个应用程序。
///
/// 它包含了应用程序的状态、事件处理器和 TUI。
pub struct App {
    running: bool,          // 表示应用程序是否正在运行
    event: EventHandler,    // 事件处理器，负责处理用户输入
    tui: RootTui,           // 根 TUI 组件
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
            event: Default::default(),
            tui: Default::default(),
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
        // 初始化终端，开启 TUI 环境
        let mut terminal = ratatui::init();

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
                    // 绘制 TUI
                    terminal.draw(|f| self.tui.render(f,f.area()))?;
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

    /// 处理按键事件，将 `KeyStatus` 映射为具体操作。
    ///
    /// # Arguments
    ///
    /// * `key_status` - 从事件处理器接收到的按键状态。
    fn event_handler(&mut self, key_status: KeyStatus) {
        match key_status {
            KeyStatus::Quit => self.stop(),                   // q → 退出程序
            KeyStatus::TogglePlay => self.tui.toggle_state(), // p → 播放/暂停
            KeyStatus::VolumeIncrease => self.tui.adjust_volume(10), // + → 增加音量
            KeyStatus::VolumeDecrease => self.tui.adjust_volume(-10), // - → 减少音量
            KeyStatus::ProgressIncrease => (),                // l → 快进
            KeyStatus::ProgressDecrease => (),                // h → 快退
            KeyStatus::PickerNext => (),                      // j → 选择下一个
            KeyStatus::PickerPrev => (),                      // k → 选择上一个
            KeyStatus::SwitchMode => (),                      // m → 切换模式
            KeyStatus::NextTrack => (),                       // ] → 下一首
            KeyStatus::PrevTrack => (),                       // [ → 上一首
            KeyStatus::PlaySelected => (),                    // Enter → 播放选中
            KeyStatus::NoOp => (),                            // 无操作
        }
    }
}