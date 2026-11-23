pub mod a0_body;
pub mod a3_body;
pub mod b6_body;

use std::io::{BufReader, Read};

use anyhow::{Context, Ok, Result};

use crate::{
    message::{a0_body::A0Body, a3_body::A3Body, b6_body::B6Body},
    utils::to_str,
};

#[derive(Debug)]
pub enum MessageBody {
    B6(B6Body),
    A3(A3Body),
    A0(A0Body),
    Other,
}

#[derive(Debug)]
pub struct Message {
    pub data_category: String,
    pub info_category: String,

    pub body: MessageBody,
}

impl Message {
    const DATA_LEN: usize = 2;
    const INFO_LEN: usize = 3;

    pub fn new(message: &Vec<u8>) -> Result<Self> {
        let mut reader = BufReader::new(message.as_slice());

        let mut data_buf: [u8; Self::DATA_LEN] = [0; Self::DATA_LEN];
        reader
            .read_exact(&mut data_buf)
            .context("reader 읽기 실패")?;
        let data = to_str(&data_buf).context("data를 String으로 변환할 수 없음")?;

        let mut info_buf: [u8; Self::INFO_LEN] = [0; Self::INFO_LEN];
        reader
            .read_exact(&mut info_buf)
            .context("reader 읽기 실패")?;
        let info = to_str(&info_buf).context("info를 String으로 변환할 수 없음")?;

        let body = match data.as_str() {
            "A3" => MessageBody::A3(
                A3Body::new(&mut reader).context("A3Body를 생성하지 못하였습니다.")?,
            ),
            "B6" => MessageBody::B6(
                B6Body::new(&mut reader).context("B6Body를 생성하지 못하였습니다.")?,
            ),
            "A0" => {
                MessageBody::A0(Body::new(&mut reader).context("A0Body를 생성하지 못하였습니다.")?)
            }
            _ => MessageBody::Other,
        };

        Ok(Self {
            data_category: data,
            info_category: info,
            body,
        })
    }
}
