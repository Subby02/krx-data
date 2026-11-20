use std::io::{BufReader, Read};

use anyhow::{Context, Result};
use rust_decimal::Decimal;

use crate::utils::{to_decimal, to_str, to_u32, to_u64};

#[derive(Debug)]
pub struct A3Body {
    pub seq_no: u32,
    pub board_id: String,
    pub session_id: String,
    pub isin: String,
    pub isin_index: u32,
    pub proc_time: String,
    pub change_sign: String,
    pub change: u64,
    pub trade_price: u64,
    pub trade_qty: u64,
    pub open: u64,
    pub high: u64,
    pub low: u64,
    pub acc_volume: u64,
    pub acc_amount: Decimal,
    pub side: String,
    pub lp_holding_qty: u64,
    pub best_ask: u64,
    pub best_bid: u64,
}

impl A3Body {
    const LENS: [usize; 19] = [
        8, 2, 2, 12, 6, 12, 1, 11, 11, 10, 11, 11, 11, 12, 22, 1, 15, 11, 11,
    ];

    pub fn new(reader: &mut BufReader<&[u8]>) -> Result<Self> {
        let mut value: Vec<Vec<u8>> = Vec::new();

        for len in Self::LENS {
            let mut buf: Vec<u8> = vec![0; len];
            reader.read_exact(&mut buf).context("reader 읽기 실패")?;

            value.push(buf);
        }

        Ok(Self {
            seq_no: to_u32(&value[0]).context("seq_no를 u32로 변환할 수 없음")?,
            board_id: to_str(&value[1]).context("board_id를 String으로 변환할 수 없음")?,
            session_id: to_str(&value[2]).context("session_id를 String으로 변환할 수 없음")?,
            isin: to_str(&value[3]).context("isin를 String으로 변환할 수 없음")?,
            isin_index: to_u32(&value[4]).context("isin_index를 u32로 변환할 수 없음")?,
            proc_time: to_str(&value[5]).context("proc_time를 String으로 변환할 수 없음")?,
            change_sign: to_str(&value[6]).context("change_sign를 String으로 변환할 수 없음")?,
            change: to_u64(&value[7]).context("change를 u64로 변환할 수 없음")?,
            trade_price: to_u64(&value[8]).context("trade_price를 u64로 변환할 수 없음")?,
            trade_qty: to_u64(&value[9]).context("trade_qty를 u64로 변환할 수 없음")?,
            open: to_u64(&value[10]).context("open를 u64로 변환할 수 없음")?,
            high: to_u64(&value[11]).context("high를 u64로 변환할 수 없음")?,
            low: to_u64(&value[12]).context("low를 u64로 변환할 수 없음")?,
            acc_volume: to_u64(&value[13]).context("acc_volume를 u64로 변환할 수 없음")?,
            acc_amount: to_decimal(&value[14]).context("acc_amount를 decimal로 변환할 수 없음")?,
            side: to_str(&value[15]).context("side를 String으로 변환할 수 없음")?,
            lp_holding_qty: to_u64(&value[16]).context("lp_holding_qty를 u64로 변환할 수 없음")?,
            best_ask: to_u64(&value[17]).context("best_ask를 u64로 변환할 수 없음")?,
            best_bid: to_u64(&value[18]).context("best_bid를 u64로 변환할 수 없음")?,
        })
    }
}
