use log::debug;

pub struct ReadLogger {
    tag: String,
    read_count: usize,
    bytes_total: usize,
}

impl ReadLogger {
    pub fn new(tag: &str) -> Self {
        debug!(",tag,length,from,to,count,bytes_total");
        ReadLogger {
            tag: tag.to_string(),
            read_count: 0,
            bytes_total: 0,
        }
    }
    pub fn log(&mut self, begin: usize, length: usize) {
        // Wraparound is ok
        self.read_count += 1;        
        self.bytes_total += length;
        debug!(
            ",{},{length},{begin},{},{},{}",
            self.tag,
            begin + length,
            self.read_count,
            self.bytes_total,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn check_stats() {
        init_logger();
        let mut stats = ReadLogger::new("READ");
        stats.log(0, 10);
        stats.log(10, 10);
        assert_eq!(stats.read_count, 2);
        assert_eq!(stats.bytes_total, 20);
    }
}
