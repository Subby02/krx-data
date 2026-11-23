use std::io::{BufReader, Read};

use anyhow::Result;

fn read_arr<const N: usize>(r: &mut impl Read) -> Result<[u8; N]> {
    let mut buf = [0u8; N];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

#[derive(Debug)]
pub struct A3Body {
    pub seq_no: [u8; 8],
    pub board_id: [u8; 2],
    pub session_id: [u8; 2],
    pub isin: [u8; 12],
    pub isin_idx: [u8; 6],
    pub proc_time: [u8; 12],
    pub change_sign: [u8; 1],
    pub change: [u8; 11],
    pub trade_price: [u8; 11],
    pub trade_qty: [u8; 10],
    pub open: [u8; 11],
    pub high: [u8; 11],
    pub low: [u8; 11],
    pub acc_volume: [u8; 12],
    pub acc_amount: [u8; 22],
    pub side: [u8; 1],
    pub lp_holding_qty: [u8; 15],
    pub best_ask: [u8; 11],
    pub best_bid: [u8; 11],
}

impl A3Body {
    pub fn new(r: &mut BufReader<&[u8]>) -> Result<Self> {
        Ok(Self {
            seq_no: read_arr(r)?,
            board_id: read_arr(r)?,
            session_id: read_arr(r)?,
            isin: read_arr(r)?,
            isin_idx: read_arr(r)?,
            proc_time: read_arr(r)?,
            change_sign: read_arr(r)?,
            change: read_arr(r)?,
            trade_price: read_arr(r)?,
            trade_qty: read_arr(r)?,
            open: read_arr(r)?,
            high: read_arr(r)?,
            low: read_arr(r)?,
            acc_volume: read_arr(r)?,
            acc_amount: read_arr(r)?,
            side: read_arr(r)?,
            lp_holding_qty: read_arr(r)?,
            best_ask: read_arr(r)?,
            best_bid: read_arr(r)?,
        })
    }
}
