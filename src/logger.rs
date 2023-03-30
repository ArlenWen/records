use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};
use crate::conf::LogConfig;

pub fn get_log_level(level: &str) -> LevelFilter {
    match level {
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Info
    }
}


pub fn init_log(log_config: &LogConfig) {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(&log_config.stdout.pattern)))
        .build();

    // let file = FileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new(&log_config.file.pattern)))
    //     .build(&log_config.file.path)
    //     .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        // .appender(Appender::builder().build("file", Box::new(file)))
        .logger(Logger::builder()
            .appender("stdout")
            .additive(false)
            .build("root", get_log_level(&log_config.level)))
        .build(Root::builder()
            .appender("stdout")
            // .appender("file")
            .build(get_log_level(&log_config.level)))
        .unwrap();

    let _ = log4rs::init_config(config).unwrap();
}