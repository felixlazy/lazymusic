use crate::types::Keymaps;
use color_eyre::eyre::{Context, Result};
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
}
