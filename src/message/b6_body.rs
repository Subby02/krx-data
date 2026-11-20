use std::io::{BufReader, Read};

use anyhow::{Context, Result};

use crate::{
    orderbook::Orderbook,
    utils::{to_str, to_u32, to_u64},
};

#[derive(Debug, Default)]
pub struct B6Body {
    pub seq_no: u32,
    pub board_id: String,
    pub session_id: String,
    pub isin: String,
    pub isin_index: u32,
    pub proc_time: String,
    pub orderbook: Orderbook,
    pub total_ask_qty: u64,
    pub total_bid_qty: u64,
    pub expected_price: u64,
    pub expected_qty: u64,
    pub mid_price: u64,
    pub mid_ask_qty: u64,
    pub mid_bid_qty: u64,
}

impl B6Body {
    const LENS: [usize; 53] = [
        8, 2, 2, 12, 6, 12, 11, 11, 12, 12, 11, 11, 12, 12, 11, 11, 12, 12, 11, 11, 12, 12, 11, 11,
        12, 12, 11, 11, 12, 12, 11, 11, 12, 12, 11, 11, 12, 12, 11, 11, 12, 12, 11, 11, 12, 12, 12,
        12, 11, 12, 11, 12, 12,
    ];

    pub fn new(reader: &mut BufReader<&[u8]>) -> Result<Self> {
        let mut value: Vec<Vec<u8>> = Vec::new();

        for len in Self::LENS {
            let mut buf: Vec<u8> = vec![0; len];
            reader.read_exact(&mut buf).context("reader 읽기 실패")?;

            value.push(buf);
        }

        let mut orderbook: Orderbook = Orderbook::default();
        for i in (6..46).step_by(4) {
            let bid_price = to_u64(&value[i]).context("bid_price를 u64로 변환할 수 없음")?;
            let ask_price = to_u64(&value[i + 1]).context("ask_price를 u64로 변환할 수 없음")?;
            let bid_qty = to_u64(&value[i + 2]).context("bid_qty를 u64로 변환할 수 없음")?;
            let ask_qty = to_u64(&value[i + 3]).context("bid_qty를 u64로 변환할 수 없음")?;
            orderbook.insert_ask(ask_price, ask_qty);
            orderbook.insert_bid(bid_price, bid_qty);
        }

        Ok(Self {
            seq_no: to_u32(&value[0]).context("seq_no를 u32로 변환할 수 없음")?,
            board_id: to_str(&value[1]).context("board_id를 String으로 변환할 수 없음")?,
            session_id: to_str(&value[2]).context("session_id를 String으로 변환할 수 없음")?,
            isin: to_str(&value[3]).context("isin를 String으로 변환할 수 없음")?,
            isin_index: to_u32(&value[4]).context("isin_index를 u32로 변환할 수 없음")?,
            proc_time: to_str(&value[5]).context("proc_time를 String으로 변환할 수 없음")?,
            orderbook,
            total_ask_qty: to_u64(&value[46]).context("total_ask_qty를 u64로 변환할 수 없음")?,
            total_bid_qty: to_u64(&value[47]).context("total_bid_qty를 u64로 변환할 수 없음")?,
            expected_price: to_u64(&value[48]).context("expected_price를 u64로 변환할 수 없음")?,
            expected_qty: to_u64(&value[49]).context("expected_qty를 u64로 변환할 수 없음")?,
            mid_price: to_u64(&value[50]).context("mid_price를 u64로 변환할 수 없음")?,
            mid_ask_qty: to_u64(&value[51]).context("mid_ask_qty를 u64로 변환할 수 없음")?,
            mid_bid_qty: to_u64(&value[52]).context("mid_bid_qty를 u64로 변환할 수 없음")?,
        })
    }
}
