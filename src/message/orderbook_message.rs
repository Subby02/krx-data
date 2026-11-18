use std::io::{BufReader, Read};

use crate::orderbook::Orderbook;

#[derive(Debug, Default)]
pub struct OrderbookMessage {
    pub data_category: String,
    pub info_category: String,
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

impl OrderbookMessage {
    const LENS: [usize; 55] = [
        2, 3, 8, 2, 2, 12, 6, 12, 11, 11, 12, 12, 11, 11, 12, 12, 11, 11, 12, 12, 11, 11, 12, 12,
        11, 11, 12, 12, 11, 11, 12, 12, 11, 11, 12, 12, 11, 11, 12, 12, 11, 11, 12, 12, 11, 11, 12,
        12, 12, 12, 11, 12, 11, 12, 12,
    ];

    pub fn new(message: &Vec<u8>) -> Self {
        let mut reader = BufReader::new(message.as_slice());
        let mut value: Vec<Vec<u8>> = Vec::new();

        for len in Self::LENS {
            let mut buf: Vec<u8> = vec![0; len];
            let _ = reader.read_exact(&mut buf);

            value.push(buf);
        }

        let mut orderbook = Orderbook::default();
        for i in (8..48).step_by(4) {
            let bid_price = std::str::from_utf8(&value[i])
                .unwrap()
                .parse::<u64>()
                .expect("bid_price를 u64로 변환할 수 없습니다.");
            let ask_price = std::str::from_utf8(&value[i + 1])
                .unwrap()
                .parse::<u64>()
                .expect("ask_price를 u64로 변환할 수 없습니다.");
            let bid_qty = std::str::from_utf8(&value[i + 2])
                .unwrap()
                .parse::<u64>()
                .expect("bid_qty를 u64로 변환할 수 없습니다.");
            let ask_qty = std::str::from_utf8(&value[i + 3])
                .unwrap()
                .parse::<u64>()
                .expect("bid_qty를 u64로 변환할 수 없습니다.");
            orderbook.insert_ask(ask_price, ask_qty);
            orderbook.insert_bid(bid_price, bid_qty);
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
            orderbook,
            total_ask_qty: std::str::from_utf8(&value[48])
                .unwrap()
                .parse::<u64>()
                .expect("total_ask_qty를 u64로 변환할 수 없습니다."),
            total_bid_qty: std::str::from_utf8(&value[49])
                .unwrap()
                .parse::<u64>()
                .expect("total_bid_qty를 u64로 변환할 수 없습니다."),
            expected_price: std::str::from_utf8(&value[50])
                .unwrap()
                .parse::<u64>()
                .expect("expected_price를 u64로 변환할 수 없습니다."),
            expected_qty: std::str::from_utf8(&value[51])
                .unwrap()
                .parse::<u64>()
                .expect("expected_qty를 u64로 변환할 수 없습니다."),
            mid_price: std::str::from_utf8(&value[52])
                .unwrap()
                .parse::<u64>()
                .expect("mid_price를 u64로 변환할 수 없습니다."),
            mid_ask_qty: std::str::from_utf8(&value[53])
                .unwrap()
                .parse::<u64>()
                .expect("mid_ask_qty를 u64로 변환할 수 없습니다."),
            mid_bid_qty: std::str::from_utf8(&value[54])
                .unwrap()
                .parse::<u64>()
                .expect("mid_bid_qty를 u64로 변환할 수 없습니다."),
        }
    }
}
