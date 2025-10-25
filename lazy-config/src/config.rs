use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
/// 应用程序配置结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct LazyConfig {
    #[serde(skip)] // 序列化/反序列化时跳过此字段
    pub path: PathBuf, // 配置文件的路径
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

        Self { path: config_path }
    }
}

impl LazyConfig {
    /// 异步读取 TOML 格式的配置文件
    ///
    /// 尝试从 `path` 参数指定的路径读取。
    /// 如果 `path` 以 `~` 开头，则会扩展为用户主目录。
    /// 如果从 `path` 读取失败，则会尝试从 `self.path`（默认配置路径）读取。
    ///
    /// # 参数
    /// - `path`: 要读取的配置文件路径，可以是相对路径、绝对路径或以 `~` 开头的路径。
    ///
    /// # 返回
    /// - `Ok(())`: 成功读取并解析配置文件。
    /// - `Err(Box<dyn Error>)`: 读取或解析配置文件失败。
    pub async fn read_toml<T: AsRef<Path>>(&mut self, path: T) -> Result<(), Box<dyn Error>> {
        let provided_path_arg = path.as_ref();
        // 处理路径中的 ~ 符号扩展
        let provided_path = if provided_path_arg.starts_with("~") {
            if let Ok(home_str) = std::env::var("HOME") {
                PathBuf::from(home_str).join(provided_path_arg.strip_prefix("~").unwrap())
            } else {
                // 如果 HOME 环境变量未找到，则不进行扩展
                provided_path_arg.to_path_buf()
            }
        } else {
            provided_path_arg.to_path_buf()
        };

        // 尝试从提供的路径读取文件内容
        let result = tokio::fs::read_to_string(&provided_path).await;

        let (contents, final_path) = match result {
            Ok(contents) => (contents, provided_path.clone()), // 成功读取，使用此路径
            Err(e_provided) => {
                // 如果从提供的路径读取失败，尝试从备用路径（self.path）读取
                let fallback_result = tokio::fs::read_to_string(&self.path).await;
                match fallback_result {
                    Ok(contents) => (contents, self.path.clone()), // 成功从备用路径读取
                    Err(e_fallback) => {
                        // 两个路径都失败，返回组合错误信息
                        return Err(format!("无法从 '{}' 读取配置 (错误: {})，也无法从备用路径 '{}' 读取 (错误: {})",
                                           provided_path.display(), e_provided,
                                           self.path.display(), e_fallback).into());
                    }
                }
            }
        };

        // 反序列化文件内容到当前结构体
        *self = toml::from_str(&contents)?;
        // 恢复正确的配置文件路径，因为反序列化会跳过 path 字段
        self.path = final_path;
        Ok(())
    }
}
