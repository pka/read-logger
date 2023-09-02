# read-logger

[![crates.io version](https://img.shields.io/crates/v/read-logger.svg)](https://crates.io/crates/read-logger)
[![docs.rs docs](https://docs.rs/read-logger/badge.svg)](https://docs.rs/read-logger)

Wrap `Read` with a read statistics logger. Implements `Read+Seek`.

## Usage example

    use std::fs::File;
    use std::io::{BufReader, Read};
    use read_logger::{Level, ReadLogger};

    let f = File::open("Cargo.toml").unwrap();
    let mut read_logger = ReadLogger::new(f, Level::Debug, "READ");
    let mut reader = BufReader::new(&mut read_logger);

    let mut bytes = [0; 4];
    reader.read_exact(&mut bytes).unwrap();
    reader.read_exact(&mut bytes).unwrap();

    // BufReader does only one read() call:
    assert_eq!(read_logger.stats().read_count, 1);
    assert!(read_logger.stats().bytes_total > 200);

Run with (using env_logger):

    RUST_LOG=read_logger=debug cargo run

Log output:

    [2023-09-02T18:41:41Z DEBUG read_logger] Initialize Read logger `READ`,tag,begin,end,length,request_length,count,bytes_total
    [2023-09-02T18:41:41Z DEBUG read_logger] Read 0-236 (237 bytes). Total requests: 1 (237 bytes),READ,0,236,237,8192,1,237
