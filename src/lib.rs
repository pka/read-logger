use log::log;
pub use log::Level;
use std::io::{Error, Read, Seek, SeekFrom};
use std::result::Result;

pub struct ReadStatsLogger {
    tag: String,
    level: Level,
    pub read_count: usize,
    pub bytes_total: usize,
}

impl ReadStatsLogger {
    pub fn new(level: Level, tag: &str) -> Self {
        log!(level, ",tag,length,from,to,count,bytes_total");
        ReadStatsLogger {
            tag: tag.to_string(),
            level,
            read_count: 0,
            bytes_total: 0,
        }
    }
    pub fn log(&mut self, begin: usize, length: usize) {
        // Wraparound is ok
        self.read_count += 1;
        self.bytes_total += length;
        log!(
            self.level,
            ",{},{length},{begin},{},{},{}",
            self.tag,
            begin + length,
            self.read_count,
            self.bytes_total,
        );
    }
}

pub struct ReadLogger<T: Read> {
    inner: T,
    logger: ReadStatsLogger,
}

impl<T: Read> ReadLogger<T> {
    pub fn new(level: Level, tag: &str, read: T) -> Self {
        ReadLogger {
            inner: read,
            logger: ReadStatsLogger::new(level, tag),
        }
    }
    pub fn stats(&self) -> &ReadStatsLogger {
        &self.logger
    }
}

impl<T: Read> Read for ReadLogger<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.logger.log(0, buf.len());
        self.inner.read(buf)
    }
}

impl<T: Read + Seek> Seek for ReadLogger<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        self.inner.seek(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn check_stats() {
        init_logger();
        let mut stats = ReadStatsLogger::new(Level::Info, "READ");
        stats.log(0, 4);
        stats.log(4, 4);
        assert_eq!(stats.read_count, 2);
        assert_eq!(stats.bytes_total, 8);
    }

    #[test]
    fn read_cursor() {
        init_logger();
        let text = "0123456789";
        let mut reader = ReadLogger::new(Level::Info, "READ", Cursor::new(text));

        let mut bytes = [0; 4];
        reader.read_exact(&mut bytes).unwrap();
        reader.read_exact(&mut bytes).unwrap();
        assert_eq!(&bytes, b"4567");
        assert_eq!(reader.stats().read_count, 2);
        assert_eq!(reader.stats().bytes_total, 8);

        let n = reader.read(&mut bytes).unwrap();
        assert_eq!(n, 2);
        // We count requested bytes, not effective bytes
        assert_eq!(reader.stats().bytes_total, 12);
    }

    #[test]
    fn seek() {
        init_logger();
        let text = "0123456789";
        let mut reader = ReadLogger::new(Level::Info, "READ", Cursor::new(text));

        let mut bytes = [0; 4];
        reader.seek(SeekFrom::Start(4)).unwrap();
        reader.read_exact(&mut bytes).unwrap();
        assert_eq!(&bytes, b"4567");
        assert_eq!(reader.stats().read_count, 1);
        assert_eq!(reader.stats().bytes_total, 4);
    }

    #[test]
    fn buf_reader() {
        init_logger();
        let text = "0123456789";
        let mut cursor = ReadLogger::new(Level::Debug, "READ", Cursor::new(text));
        // To be able to access stats after reading, we borrow cursor to BufReader
        let mut buffer = ReadLogger::new(Level::Info, "BUFFER", BufReader::new(&mut cursor));

        let mut bytes = [0; 4];
        buffer.read_exact(&mut bytes).unwrap();
        buffer.read_exact(&mut bytes).unwrap();
        assert_eq!(&bytes, b"4567");
        assert_eq!(buffer.stats().read_count, 2);
        assert_eq!(buffer.stats().bytes_total, 8);
        assert_eq!(cursor.stats().read_count, 1);
        assert_eq!(cursor.stats().bytes_total, 8192);
    }
}
