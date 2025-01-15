use chrono::Local;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct Logger {
    log_sink: File,
}

impl Logger {
    pub fn new(sink: File) -> Self {
        Self { log_sink: sink }
    }
}

impl Logger {
    pub async fn log(&mut self, msg: &str) -> Result<(), std::io::Error> {
        let now = Local::now();
        let log_msg = format!("[{}] {}\n", now.format("%d-%m-%Y %H:%M:%S"), msg);
        self.log_sink.write_all(log_msg.as_bytes()).await?;
        Ok(())
    }
}
