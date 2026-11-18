use crate::message::Message;
use crate::record::Record;
use chrono::NaiveTime;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::path::Path;

pub struct DumpReader {
    reader: BufReader<File>,
}

impl DumpReader {
    pub fn new(path: &str) -> Result<DumpReader, Error> {
        let path = Path::new(path);
        let file = File::open(path)?;

        Ok(Self {
            reader: BufReader::new(file),
        })
    }
}

impl Iterator for DumpReader {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reader.fill_buf().unwrap().is_empty() {
            None
        } else {
            //size
            let mut size_buf: Vec<u8> = vec![0; Record::SIZE_LEN + 1];
            let _ = self.reader.read_exact(&mut size_buf);
            let size = String::from_utf8(size_buf[..Record::SIZE_LEN].to_vec())
                .unwrap()
                .parse::<usize>()
                .expect("invalid digit");

            //time
            let mut time_buf: Vec<u8> = vec![0; Record::TIME_LEN + 1];
            let _ = self.reader.read_exact(&mut time_buf);
            let time = NaiveTime::parse_from_str(
                &str::from_utf8(&time_buf[..Record::TIME_LEN].to_vec()).unwrap(),
                "%H%M%S%6f",
            )
            .expect("invalid time");

            //message
            let mut msg: Vec<u8> = vec![0; size - 1];
            let _ = self.reader.read_exact(&mut msg);
            let message = Message::new(msg);

            // 짧은 전문이 들어오면 \r\n 형태로 끝나서 whitespace 형태는 스킵
            while self
                .reader
                .fill_buf()
                .unwrap()
                .first()
                .is_some_and(|b| b.is_ascii_whitespace())
            {
                self.reader.consume(1);
            }

            Some(Record {
                size,
                time,
                message,
            })
        }
    }
}
