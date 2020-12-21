mod logging;

static LOG: logging::SimpleLogger = logging::SimpleLogger {
    log_level: logging::Level::Info
};

fn main() {
    logging::set_logger(&LOG)
        .map(|()| logging::set_max_level(logging::LevelFilter::Trace)).unwrap();
    
    logging::bootup();
}