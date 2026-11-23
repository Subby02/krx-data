use std::io::{BufReader, Read};

use anyhow::Result;

use crate::orderbook::Orderbook;

fn read_arr<const N: usize>(r: &mut impl Read) -> Result<[u8; N]> {
    let mut buf = [0u8; N];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

#[derive(Debug, Default)]
pub struct B6Body {
    pub seq_no: [u8; 8],
    pub board_id: [u8; 2],
    pub session_id: [u8; 2],
    pub isin: [u8; 12],
    pub isin_idx: [u8; 6],
    pub proc_time: [u8; 12],
    pub orderbook: Orderbook,
    pub total_ask_qty: [u8; 12],
    pub total_bid_qty: [u8; 12],
    pub expected_price: [u8; 11],
    pub expected_qty: [u8; 12],
    pub mid_price: [u8; 11],
    pub mid_ask_qty: [u8; 12],
    pub mid_bid_qty: [u8; 12],
}

impl B6Body {
    pub fn new(r: &mut BufReader<&[u8]>) -> Result<Self> {
        let mut orderbook: Orderbook = Orderbook::default();
        let mut result = Self {
            seq_no: read_arr(r)?,
            board_id: read_arr(r)?,
            session_id: read_arr(r)?,
            isin: read_arr(r)?,
            isin_idx: read_arr(r)?,
            proc_time: read_arr(r)?,
            ..Default::default()
        };

        for (ask, bid) in orderbook.asks.iter_mut().zip(orderbook.bids.iter_mut()) {
            ask.price = read_arr(r)?;
            bid.price = read_arr(r)?;
            ask.qty = read_arr(r)?;
            bid.qty = read_arr(r)?;
        }

        result = Self {
            orderbook,
            total_ask_qty: read_arr(r)?,
            total_bid_qty: read_arr(r)?,
            expected_price: read_arr(r)?,
            expected_qty: read_arr(r)?,
            mid_price: read_arr(r)?,
            mid_ask_qty: read_arr(r)?,
            mid_bid_qty: read_arr(r)?,
            ..result
        };

        Ok(result)
    }
}
