extern crate env_logger;

use std::env;
use std::ffi::CString;
use std::io::Write;
use std::ptr;

use chrono::Local;
use env_logger::fmt::Formatter;
use log::{info, LevelFilter, Record};

use self::env_logger::Builder as EnvLoggerBuilder;

fn format_text_no_color(buf: &mut Formatter, record: &Record) -> std::io::Result<()> {
    writeln!(buf, "{:>5}|{:}|{:>25}:{:<4}| {}",
             record.level(),
             Local::now().format("%Y-%m-%d %H:%M:%S.%f"),
             record.file().get_or_insert(""),
             record.line().get_or_insert(0),
             record.args()
    )
}

fn format_text_color(buf: &mut Formatter, record: &Record) -> std::io::Result<()> {
    let level_colorized = buf.default_styled_level(record.level());
    writeln!(buf, "{:>5}|{:}|{:>25}:{:<4}| {}",
             level_colorized,
             Local::now().format("%Y-%m-%d %H:%M:%S.%f"),
             record.file().get_or_insert(""),
             record.line().get_or_insert(0),
             record.args()
    )
}

pub fn init_logger(pattern: String, log_format: String) -> Result<(), anyhow::Error> {
    info!("init_logger >>> pattern: {}, log_format: {}", pattern, log_format);

    let format = match log_format.as_str() {
        "text_no_color" => format_text_no_color,
        _ => format_text_color,
    };
    EnvLoggerBuilder::new()
        .format(format)
        .filter(None, LevelFilter::Off)
        .parse_filters(&pattern)
        .try_init()
        .map_err(|err| anyhow!("Cannot init logger: {:?}", err))
}
