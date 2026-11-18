pub mod orderbook_message;
pub mod trade_message;

use std::io::{BufReader, Read};

use crate::message::{orderbook_message::OrderbookMessage, trade_message::TradeMessage};

#[derive(Debug)]
pub enum MessageKind {
    Orderbook(OrderbookMessage),
    Trade(TradeMessage),
    Other,
}

#[derive(Debug)]
pub struct Message {
    pub kind: MessageKind,
}

impl Message {
    pub fn new(message: Vec<u8>) -> Self {
        let mut reader = BufReader::new(message.as_slice());

        let mut kind_buf: [u8; 2] = [0; 2];
        let _ = reader.read_exact(&mut kind_buf);

        match kind_buf {
            [b'A', b'3'] => Self {
                kind: MessageKind::Trade(TradeMessage::new(&message)),
            },
            [b'B', b'6'] => Self {
                kind: MessageKind::Orderbook(OrderbookMessage::new(&message)),
            },
            _ => Self {
                kind: MessageKind::Other,
            },
        }
    }
}
