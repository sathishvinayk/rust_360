//Log module
pub use log::{set_logger, set_max_level, Level, LevelFilter};
use crate::logging::logging_module::*;

#[derive(Debug, PartialEq)]
pub enum Switch {
    REGISTER,
    UNREGISTER
}

pub const LOG: SimpleLogger = SimpleLogger {
    log_level: Level::Info
};

mod logging_module {
    use log::{Level, error, info, Metadata, Record};
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
        pub fn start_logging(){
            info!("Information: Starting off the raspberry pi now...!");
            error!("Peripherals not connected!");
        }
    }
}

pub fn bootup(status: Switch) {
    if status == Switch::REGISTER {
        logging_module::SimpleLogger::start_logging()
    } else {
        println!("Loggin not enabled, status: {:?}", Switch::UNREGISTER);
    }
}