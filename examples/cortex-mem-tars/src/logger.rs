use anyhow::{Context, Result};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tracing::{Level, Subscriber};
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

/// 日志管理器（保持向后兼容的接口）
pub struct LogManager {
    #[allow(dead_code)]
    log_file: PathBuf,
    file: Arc<Mutex<File>>,
    lines: Arc<Mutex<Vec<String>>>,
}

impl LogManager {
    /// 创建新的日志管理器
    pub fn new(log_dir: &Path) -> Result<Self> {
        let log_file = log_dir.join("app.log");

        // 确保日志目录存在
        if let Some(parent) = log_file.parent() {
            std::fs::create_dir_all(parent).context("无法创建日志目录")?;
        }

        // 打开或创建日志文件（追加模式）
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .context("无法打开日志文件")?;

        Ok(Self {
            log_file,
            file: Arc::new(Mutex::new(file)),
            lines: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// 读取内存中的最近日志
    pub fn read_logs(&self, max_lines: usize) -> Result<Vec<String>> {
        let lines = self
            .lines
            .lock()
            .map_err(|e| anyhow::anyhow!("无法获取日志行锁: {}", e))?;

        if lines.len() > max_lines {
            Ok(lines[lines.len() - max_lines..].to_vec())
        } else {
            Ok(lines.clone())
        }
    }
}

/// 自定义 tracing Layer，把日志同时写入文件和内存缓冲
struct FileLayer {
    manager: Arc<LogManager>,
}

impl<S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>> Layer<S>
    for FileLayer
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let meta = event.metadata();
        let level = meta.level();

        // INFO 级别以上才写入（过滤 DEBUG/TRACE 减少噪音）
        if *level > Level::INFO {
            return;
        }

        // 提取消息和来自 log-bridge 的额外字段
        let mut visitor = FullVisitor::default();
        event.record(&mut visitor);

        // target 优先级：
        //   1. 若是 tracing-log bridge 转来的事件，target 就是 `"log"`，
        //      此时使用 bridge 附加的 `log.module_path` 字段作为真实来源
        //   2. 否则直接使用 metadata 中的 target（即 tracing 宏调用点的模块路径）
        let target = if meta.target() == "log" {
            visitor
                .log_module_path
                .as_deref()
                .unwrap_or("tars")
                .to_string()
        } else {
            meta.target().to_string()
        };

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let log_line = format!(
            "[{} {}] [{}] {}",
            timestamp, level, target, visitor.message
        );

        // 写入文件（静默失败，不干扰 TUI）
        if let Ok(mut file) = self.manager.file.lock() {
            let _ = writeln!(file, "{}", log_line);
            let _ = file.flush();
        }

        // 写入内存缓冲
        if let Ok(mut lines) = self.manager.lines.lock() {
            lines.push(log_line);
            if lines.len() > 1000 {
                let excess = lines.len() - 1000;
                lines.drain(0..excess);
            }
        }
    }
}

/// 从 tracing Event 中提取所有关心的字段：
/// - `message`       — 日志正文
/// - `log.module_path` — tracing-log bridge 附加的来源模块路径
/// - `log.target`    — tracing-log bridge 附加的原始 target（备用）
#[derive(Default)]
struct FullVisitor {
    message: String,
    log_module_path: Option<String>,
}

impl tracing::field::Visit for FullVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "message" => self.message.push_str(value),
            "log.module_path" => self.log_module_path = Some(value.to_string()),
            _ => {}
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            use std::fmt::Write;
            let _ = write!(self.message, "{:?}", value);
        }
    }
}

/// 初始化日志系统
///
/// 统一用 `tracing` 生态作为日志后端：
/// - `cortex-mem-core` 等子 crate 使用 `tracing::info!` → 直接被 subscriber 捕获
/// - tars 自身使用 `log::info!`    → 通过 `tracing_log::LogTracer` 转发给 tracing
///
/// 所有日志都写入同一个 `app.log` 文件，并在内存中保留最近 1000 条供查阅。
///
/// 可通过环境变量 `RUST_LOG` 覆盖默认级别（例如 `RUST_LOG=debug` 开启详细输出）。
pub fn init_logger(log_dir: &Path) -> Result<Arc<LogManager>> {
    let manager = Arc::new(LogManager::new(log_dir)?);

    // 桥接 `log` → tracing：让 log::info! / log::warn! 等也被 tracing 捕获。
    // with_max_level 设为 INFO，过滤掉 log crate 产生的 DEBUG/TRACE 事件。
    tracing_log::LogTracer::builder()
        .with_max_level(log::LevelFilter::Info)
        .init()
        .ok(); // 若已初始化则忽略（防止测试环境重复 init 报错）

    let file_layer = FileLayer {
        manager: Arc::clone(&manager),
    };

    // 环境变量 RUST_LOG 可覆盖默认级别
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .try_init()
        .ok(); // 若已初始化则忽略

    tracing::info!("日志系统初始化完成");
    tracing::info!("日志文件路径: {}", log_dir.display());

    Ok(manager)
}
