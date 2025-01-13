use chrono::Local;
use std::{env, fs::File, io::Write};

const PREFIX_PATH: &str = "/root";

pub struct Logger<W> {
    log_sink: W,
}

impl<W> Logger<W> {
    pub fn new(sink: W) -> Self {
        Self { log_sink: sink }
    }
}

impl<W: Write> Logger<W> {
    pub fn log(&mut self, msg: &str) -> Result<(), std::io::Error> {
        let now = Local::now();
        writeln!(
            self.log_sink,
            "[{}] {}",
            now.format("%d-%m-%Y %H:%M:%S"),
            msg
        )?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hostname = env::var("HOSTNAME")?;
    let log_file = File::create(format!("{PREFIX_PATH}/{hostname}.log"))?;
    let mut logger = Logger::new(log_file);

    logger.log(&format!("{hostname} started running."))?;
    Ok(())
}
