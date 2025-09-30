use std::error::Error;

use lazy_tui::{root::RootTui, traits::RenderTui};
use tokio::time::{Duration, Interval, MissedTickBehavior, interval};

use crate::event::{EventHandler, KeyStatus};
pub struct App {
    running: bool,
    event: EventHandler,
    tui: RootTui,
    tui_interval: Interval,
}
impl Default for App {
    fn default() -> Self {
        let mut tui_interval = interval(Duration::from_millis(100));
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
    /// 异步运行主循环
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
                    // 这里绘制 UI（当前例子为空实现）
                    terminal.draw(|f| self.tui.render(f,f.area()))?;
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
