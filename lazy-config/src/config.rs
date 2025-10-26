use crate::keymap::{ActionArgument, KeymapConfig, Keymaps};
use color_eyre::eyre::{Context, Result};
use lazy_core::types::KeyStatus;
use serde::{Deserialize, Serialize};
use std::{
    env,
    path::{Path, PathBuf},
};

/// 应用程序配置结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct LazyConfig {
    /// 配置文件的路径
    #[serde(skip)] // 序列化/反序列化时跳过此字段
    pub path: PathBuf,
    /// 键位映射配置
    pub keymap: Option<Keymaps>,
}

impl Default for LazyConfig {
    /// 提供 LazyConfig 的默认实现
    fn default() -> Self {
        // 尝试从 XDG_CONFIG_HOME 环境变量获取配置目录
        let mut config_path = if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_config_home)
        // 如果 XDG_CONFIG_HOME 不存在，则尝试从 HOME 环境变量获取，并拼接 .config
        } else if let Ok(home) = env::var("HOME") {
            PathBuf::from(home).join(".config")
        // 如果以上都失败，则回退到当前目录
        } else {
            PathBuf::from(".")
        };
        // 拼接具体的配置文件名
        config_path.push("lazymusic/config.toml");

        Self {
            path: config_path,
            keymap: Default::default(),
        }
    }
}

impl LazyConfig {
    /// 从指定路径加载配置。
    ///
    /// - 如果 `path` 为 `Some`，则会尝试读取该路径。
    ///   - 支持 `~` 开头的路径扩展。
    ///   - 如果读取失败，会回退到默认配置路径。
    /// - 如果 `path` 为 `None`，则只尝试读取默认配置路径。
    pub async fn load(path: Option<&Path>) -> Result<Self> {
        let default_path = Self::default().path;

        // 处理用户提供的路径，包括 `~` 扩展
        let user_path = path.map(|p| {
            if p.starts_with("~") {
                env::var("HOME")
                    .map(|home| PathBuf::from(home).join(p.strip_prefix("~").unwrap()))
                    .unwrap_or_else(|_| p.to_path_buf())
            } else {
                p.to_path_buf()
            }
        });

        // 根据路径读取文件内容，并实现回退逻辑
        let (contents, final_path) = if let Some(p) = user_path {
            // 优先读取用户指定的路径
            match tokio::fs::read_to_string(&p).await {
                Ok(c) => (c, p),
                Err(e_user) => {
                    // 如果主要路径失败，则尝试备用路径。
                    // `?` 会在备用路径也失败时，将带有完整上下文的错误向上传播。
                    let fallback_contents = tokio::fs::read_to_string(&default_path)
                        .await
                        .wrap_err_with(|| {
                            format!(
                                "尝试主要路径 '{}' (失败: {}) 后，读取备用路径 '{}' 也失败",
                                p.display(),
                                e_user,
                                default_path.display()
                            )
                        })?;
                    (fallback_contents, default_path)
                }
            }
        } else {
            // 如果没有提供路径，则只读取默认路径
            let contents = tokio::fs::read_to_string(&default_path)
                .await
                .wrap_err_with(|| {
                    format!("无法从默认路径 '{}' 读取配置", default_path.display())
                })?;
            (contents, default_path)
        };

        // 从文件内容反序列化
        let mut config: Self = toml::from_str(&contents).wrap_err("解析 TOML 文件内容失败")?;

        // 恢复正确的配置文件路径
        config.path = final_path;

        Ok(config)
    }

    /// 如果默认配置文件不存在，则创建并写入一组默认配置。
    ///
    /// # 返回
    ///
    /// - `Ok(PathBuf)`: 如果文件已存在或成功创建，则返回配置文件的路径。
    /// - `Err(eyre::Report)`: 如果创建目录或写入文件失败。
    pub async fn write_default_if_not_exists() -> Result<PathBuf> {
        let mut config = Self::default();
        let path = config.path.clone();

        // 如果文件已存在，则不执行任何操作
        if path.exists() {
            return Ok(path);
        }

        // 创建一组合理的默认键位映射
        let default_keymaps = Keymaps {
            configs: vec![
                KeymapConfig {
                    on: "q".to_string(),
                    run: KeyStatus::Quit,
                    argument: None,
                    desc: Some("退出程序".to_string()),
                },
                KeymapConfig {
                    on: "+".to_string(),
                    run: KeyStatus::VolumeIncrease,
                    argument: Some(ActionArgument::Value(10)),
                    desc: Some("音量增加 10".to_string()),
                },
                KeymapConfig {
                    on: "-".to_string(),
                    run: KeyStatus::VolumeDecrease,
                    argument: Some(ActionArgument::Value(10)),
                    desc: Some("音量减少 10".to_string()),
                },
                KeymapConfig {
                    on: "L".to_string(),
                    run: KeyStatus::NavbarNext,
                    argument: None,
                    desc: Some("下一个选项".to_string()),
                },
                KeymapConfig {
                    on: "H".to_string(),
                    run: KeyStatus::NavbarPrev,
                    argument: None,
                    desc: Some("上一个选项".to_string()),
                },
                KeymapConfig {
                    on: "j".to_string(),
                    run: KeyStatus::PickerNext,
                    argument: None,
                    desc: Some("下一个选项".to_string()),
                },
                KeymapConfig {
                    on: "k".to_string(),
                    run: KeyStatus::PickerPrev,
                    argument: None,
                    desc: Some("上一个选项".to_string()),
                },
                KeymapConfig {
                    on: "]".to_string(),
                    run: KeyStatus::NextTrack,
                    argument: None,
                    desc: Some("上一首".to_string()),
                },
                KeymapConfig {
                    on: "[".to_string(),
                    run: KeyStatus::PrevTrack,
                    argument: None,
                    desc: Some("下一首".to_string()),
                },
                KeymapConfig {
                    on: "m".to_string(),
                    run: KeyStatus::SwitchMode,
                    argument: None,
                    desc: Some("切换模式".to_string()),
                },
                KeymapConfig {
                    on: "l".to_string(),
                    run: KeyStatus::ProgressIncrease,
                    argument: Some(ActionArgument::Value(10)),
                    desc: Some("进度增加 10s".to_string()),
                },
                KeymapConfig {
                    on: "h".to_string(),
                    run: KeyStatus::ProgressDecrease,
                    argument: Some(ActionArgument::Value(10)),
                    desc: Some("进度减少 10s".to_string()),
                },
                KeymapConfig {
                    on: "<enter>".to_string(),
                    run: KeyStatus::PlaySelected,
                    argument: None,
                    desc: Some("播放选中的".to_string()),
                },
                KeymapConfig {
                    on: "p".to_string(),
                    run: KeyStatus::TogglePlay,
                    argument: None,
                    desc: Some("切换播放".to_string()),
                },
            ],
        };
        config.keymap = Some(default_keymaps);

        // 确保父目录存在
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .wrap_err("无法创建配置目录")?;
        }

        // 序列化为格式化的 TOML 字符串
        let toml_string = toml::to_string_pretty(&config).wrap_err("无法序列化默认配置")?;

        // 写入文件
        tokio::fs::write(&path, toml_string)
            .await
            .wrap_err("无法写入默认配置文件")?;

        Ok(path)
    }
}
