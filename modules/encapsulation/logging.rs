//Log module
pub use log::*;

pub struct SimpleLogger {
    pub log_level: Level
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

impl SimpleLogger {
    fn start_logging(){
        info!("Information: Starting off the raspberry pi now...!");
        error!("Peripherals not connected!");
    }
}

pub fn bootup() {
    SimpleLogger::start_logging();
}