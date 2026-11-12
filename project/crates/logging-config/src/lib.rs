use std::io::Write;

pub fn configure_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buffer, record| {
            writeln!(
                buffer,
                "{} {:5} [{}] {}",
                buffer.timestamp_millis(),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();
}
