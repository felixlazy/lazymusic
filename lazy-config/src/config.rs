use crate::types::Keymaps;
use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
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
    ///
    /// # 参数
    /// - `path`: `Option<&Path>`，要读取的配置文件路径。
    ///
    /// # 返回
    /// - `Ok(Self)`: 成功加载并解析的 `LazyConfig` 实例。
    /// - `Err(Box<dyn Error>)`: 读取或解析失败。
    pub async fn load(path: Option<&Path>) -> Result<Self, Box<dyn Error>> {
        let default_conf = Self::default();
        let default_path = &default_conf.path;

        // 处理用户提供的路径，包括 `~` 扩展
        let user_path = path.map(|p| {
            if p.starts_with("~") {
                if let Ok(home) = std::env::var("HOME") {
                    PathBuf::from(home).join(p.strip_prefix("~").unwrap())
                } else {
                    p.to_path_buf() // HOME 未设置，按原样使用路径
                }
            } else {
                p.to_path_buf()
            }
        });

        // 根据路径读取文件内容，并实现回退逻辑
        let (contents, final_path) = if let Some(p) = user_path {
            // 优先读取用户指定的路径
            match tokio::fs::read_to_string(&p).await {
                Ok(c) => (c, p), // 成功
                Err(e_user) => {
                    // 失败，则尝试回退到默认路径
                    match tokio::fs::read_to_string(default_path).await {
                        Ok(c) => (c, default_path.to_path_buf()),
                        Err(e_default) => {
                            // 两个路径都失败，返回组合错误信息
                            return Err(format!(
                                "无法从 '{}' 读取配置 (错误: {})，也无法从备用路径 '{}' 读取 (错误: {})",
                                p.display(),
                                e_user,
                                default_path.display(),
                                e_default
                            )
                            .into());
                        }
                    }
                }
            }
        } else {
            // 如果没有提供路径，则只读取默认路径
            match tokio::fs::read_to_string(default_path).await {
                Ok(c) => (c, default_path.to_path_buf()),
                Err(e) => {
                    return Err(format!(
                        "无法从默认路径 '{}' 读取配置 (错误: {})",
                        default_path.display(),
                        e
                    )
                    .into());
                }
            }
        };

        // 从文件内容反序列化
        let mut config: Self = toml::from_str(&contents)?;
        // 恢复正确的配置文件路径
        config.path = final_path;

        Ok(config)
    }
}
