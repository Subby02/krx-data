use std::{
    io::{BufReader, Read},
    str::FromStr,
};

use rust_decimal::Decimal;

#[derive(Debug)]
pub struct TradeMessage {
    pub data_category: String,
    pub info_category: String,
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

impl TradeMessage {
    const LENS: [usize; 21] = [
        2, 3, 8, 2, 2, 12, 6, 12, 1, 11, 11, 10, 11, 11, 11, 12, 22, 1, 15, 11, 11,
    ];

    pub fn new(message: &Vec<u8>) -> Self {
        let mut reader = BufReader::new(message.as_slice());
        let mut value: Vec<Vec<u8>> = Vec::new();

        for len in Self::LENS {
            let mut buf: Vec<u8> = vec![0; len];
            let _ = reader.read_exact(&mut buf);

            value.push(buf);
        }

        Self {
            data_category: String::from_utf8_lossy(&value[0]).into_owned(),
            info_category: String::from_utf8_lossy(&value[1]).into_owned(),
            seq_no: std::str::from_utf8(&value[2])
                .unwrap()
                .parse::<u32>()
                .expect("seq_no를 u32로 변환할 수 없습니다."),
            board_id: String::from_utf8_lossy(&value[3]).into_owned(),
            session_id: String::from_utf8_lossy(&value[4]).into_owned(),
            isin: String::from_utf8_lossy(&value[5]).into_owned(),
            isin_index: std::str::from_utf8(&value[6])
                .unwrap()
                .parse::<u32>()
                .expect("isin_index를 u32로 변환할 수 없습니다."),
            proc_time: String::from_utf8_lossy(&value[7]).into_owned(),
            change_sign: String::from_utf8_lossy(&value[8]).into_owned(),
            change: std::str::from_utf8(&value[9])
                .unwrap()
                .parse::<u64>()
                .expect("change를 u64로 변환할 수 없습니다."),
            trade_price: std::str::from_utf8(&value[10])
                .unwrap()
                .parse::<u64>()
                .expect("change를 u64로 변환할 수 없습니다."),
            trade_qty: std::str::from_utf8(&value[11])
                .unwrap()
                .parse::<u64>()
                .expect("change를 u64로 변환할 수 없습니다."),
            open: std::str::from_utf8(&value[12])
                .unwrap()
                .parse::<u64>()
                .expect("change를 u64로 변환할 수 없습니다."),
            high: std::str::from_utf8(&value[13])
                .unwrap()
                .parse::<u64>()
                .expect("change를 u64로 변환할 수 없습니다."),
            low: std::str::from_utf8(&value[14])
                .unwrap()
                .parse::<u64>()
                .expect("change를 u64로 변환할 수 없습니다."),
            acc_volume: std::str::from_utf8(&value[15])
                .unwrap()
                .parse::<u64>()
                .expect("change를 u64로 변환할 수 없습니다."),
            acc_amount: Decimal::from_str(std::str::from_utf8(&value[16]).unwrap()).unwrap(),
            side: String::from_utf8_lossy(&value[17]).into_owned(),
            lp_holding_qty: std::str::from_utf8(&value[18])
                .unwrap()
                .parse::<u64>()
                .expect("change를 u64로 변환할 수 없습니다."),
            best_ask: std::str::from_utf8(&value[19])
                .unwrap()
                .parse::<u64>()
                .expect("change를 u64로 변환할 수 없습니다."),
            best_bid: std::str::from_utf8(&value[20])
                .unwrap()
                .parse::<u64>()
                .expect("change를 u64로 변환할 수 없습니다."),
        }
    }
}
