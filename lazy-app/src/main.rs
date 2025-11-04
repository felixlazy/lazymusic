use lazy_app::app::App;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // 安装 color_eyre 错误报告钩子，提供更友好的错误输出
    color_eyre::install()?;

    // 配置日志文件滚动策略：每天生成一个名为 "lazymusic_log" 的日志文件，存放在 "logs" 目录下
    let file_appender = rolling::daily("logs", "lazymusic_log");
    // 创建非阻塞日志写入器，避免日志写入阻塞主线程
    let (bot_blocking, _guard) = non_blocking(file_appender);

    // 创建一个格式化层，用于将日志写入文件
    // .with_writer(bot_blocking): 指定日志写入目标为非阻塞文件写入器
    // .with_ansi(false): 禁用 ANSI 颜色代码，确保日志文件内容纯净
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(bot_blocking)
        .with_ansi(false);

    // 构建并初始化全局 tracing 订阅器
    tracing_subscriber::registry()
        // 添加文件日志层
        .with(file_layer)
        // 添加环境变量过滤器，允许通过环境变量（如 RUST_LOG）控制日志级别
        // 如果未设置环境变量，则默认使用 TRACE 级别
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("TRACE")),
        )
        // 添加 tracing-error 的 ErrorLayer，用于捕获和报告错误
        .with(tracing_error::ErrorLayer::default())
        // 初始化订阅器，使其生效
        .init();

    // 运行应用程序的主逻辑
    let result = App::default().run().await;

    // 如果应用程序运行过程中返回错误，则将错误信息记录到日志中
    if let Err(ref e) = result {
        tracing::error!("Application exited with error: {:#?}", e);
    }

    // 返回应用程序的最终结果
    result
}
