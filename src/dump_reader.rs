use crate::message::Message;
use crate::record::Record;
use crate::utils::{to_str, to_usize};
use anyhow::{Context, Result};
use chrono::NaiveTime;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

pub struct DumpReader {
    reader: BufReader<File>,
}

impl DumpReader {
    pub fn new(path: &str) -> Result<DumpReader> {
        let path = Path::new(path);
        let file = File::open(path)?;

        Ok(Self {
            reader: BufReader::new(file),
        })
    }
}

impl Iterator for DumpReader {
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.fill_buf() {
            Ok([]) => return None,
            Ok(_) => {}
            Err(e) => return Some(Err(e).context("버퍼 읽기 실패")),
        }

        //size
        let mut size_buf: Vec<u8> = vec![0; Record::SIZE_LEN + 1];
        if let Err(e) = self.reader.read_exact(&mut size_buf) {
            return Some(Err(e).context("size 읽기 실패"));
        }
        let size = to_usize(&size_buf[..Record::SIZE_LEN]).ok()?;

        //time
        let mut time_buf: Vec<u8> = vec![0; Record::TIME_LEN + 1];
        if let Err(e) = self.reader.read_exact(&mut time_buf) {
            return Some(Err(e).context("time 읽기 실패"));
        }
        let time = match NaiveTime::parse_from_str(
            to_str(&time_buf[..Record::TIME_LEN]).ok()?.as_str(),
            "%H%M%S%6f",
        ) {
            Ok(time) => time,
            Err(e) => return Some(Err(e).context("NaiveTime 변환 실패")),
        };

        //message
        let mut msg: Vec<u8> = vec![0; size - 1];
        if let Err(e) = self.reader.read_exact(&mut msg) {
            return Some(Err(e).context("msg 읽기 실패"));
        }
        let message = match Message::new(&msg) {
            Ok(msg) => msg,
            Err(e) => return Some(Err(e).context("Message 생성 실패")),
        };

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

        Some(Ok(Record {
            size,
            time,
            message,
        }))
    }
}
