pub use env_logger;

pub fn log_format_clean(
    buf: &mut env_logger::fmt::Formatter,
    record: &log::Record<'_>,
) -> std::io::Result<()> {
    use std::io::Write;

    writeln!(
        buf,
        "{} [{}] - {}",
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S"),
        record.level(),
        record.args(),
    )
}

pub fn log_format_full(
    buf: &mut env_logger::fmt::Formatter,
    record: &log::Record<'_>,
) -> std::io::Result<()> {
    use std::io::Write;

    writeln!(
        buf,
        "{} [{}] - {}, {}:{}",
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S"),
        record.level(),
        record.args(),
        record.file().unwrap_or("unknown"),
        record.line().unwrap_or(0),
    )
}
