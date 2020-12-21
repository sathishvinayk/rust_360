mod logging;

fn main() {
    let status: logging::Switch = logging::Switch::UNREGISTER;
    logging::set_logger(&logging::LOG)
        .map(|()| logging::set_max_level(logging::LevelFilter::Trace)).unwrap();
    
    logging::bootup(status);
}