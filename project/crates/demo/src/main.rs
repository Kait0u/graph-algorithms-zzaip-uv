use log::info;

fn main() {
    logging_config::configure_logging();

    info!("Hello, world!");
}
