use std::env;

/// 日志处理，重载了 log::Log trait

use log::Level;

struct DiskpineLogger;

/// 初始化 log，从 .env 文件中读取最低日志级别 LOG_LEVEL 环境变量，设置默认级别为 INFO
pub fn log_init() {
    let log_level = env::var("MINI_REDIS_LOG").unwrap_or_else(|_| String::from("TRACE"));
    let log_level = match log_level.as_str() {
        "OFF" => log::LevelFilter::Off,
        "ERROR" => log::LevelFilter::Error,
        "WARN" => log::LevelFilter::Warn,
        "INFO" => log::LevelFilter::Info,
        "DEBUG" => log::LevelFilter::Debug,
        "TRACE" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };

    static LOGGER: DiskpineLogger = DiskpineLogger;
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log_level))
        .expect("log_level set error!");
}

impl log::Log for DiskpineLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        // metadata.level() <= Level::Info
        true
    }

    /// 重载 log trait，实现打印颜色
    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            Level::Error => 31, // 红色
            Level::Warn => 93,  // 亮黄色
            Level::Info => 32,  // 绿色
            Level::Debug => 34, // 蓝色
            Level::Trace => 90, // 浅灰色
        };

        println!(
            "\u{1B}[{}m[{:>5}]: {} - {}\u{1B}[0m",
            color,
            record.level(),
            record.target(),
            record.args(),
        );
    }

    fn flush(&self) {}
}
