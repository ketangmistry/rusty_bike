use std::io::Error;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::runtime::ConfigErrors;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn get_logger_file() -> Result<FileAppender, Error> {
    FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("/Volumes/Extra/Temps/rusty_bike.log")
}

pub fn init_logger() -> Result<log4rs::Config, ConfigErrors> {
    match get_logger_file() {
        Ok(logger_file) => Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logger_file)))
            .build(Root::builder().appender("logfile").build(LevelFilter::Info)),
        Err(_) => todo!(),
    }
}
