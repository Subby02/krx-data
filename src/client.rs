use std::io::{Write, stdout};

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, Stylize};

use crate::message::{Message, MessageKind};
use crate::orderbook::Orderbook;

struct Trade {
    price: u64,
    qty: u64,
}

pub struct Client {
    orderbook: Orderbook,
    trades: Vec<Trade>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            orderbook: Orderbook::default(),
            trades: Vec::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message.kind {
            MessageKind::Orderbook(message) => {
                self.orderbook = message.orderbook;
            }
            MessageKind::Trade(message) => {
                self.trades.push(Trade {
                    price: message.trade_price,
                    qty: message.trade_qty,
                });
            }
            MessageKind::Other => {}
        }
    }

    pub fn show(&self) {
        let mut out = stdout();

        for (i, bid) in self.orderbook.bids.iter().enumerate() {
            let y = 9 - i as u16;
            queue!(
                out,
                MoveTo(0, y),
                Print(format!("{:<12}", bid.qty).blue()),
                MoveTo(13, y),
                Print(format!("{:^11}", bid.price).blue()),
            )
            .unwrap();
        }

        for (i, ask) in self.orderbook.asks.iter().enumerate() {
            let y = 10 + i as u16;
            queue!(
                out,
                MoveTo(13, y),
                Print(format!("{:^11}", ask.price).red()),
                MoveTo(25, y),
                Print(format!("{:>12}", ask.qty).red()),
            )
            .unwrap();
        }

        queue!(
            out,
            MoveTo(13, 21),
            Print(match self.trades.last() {
                Some(trade) => format!("{:^11}", trade.price),
                None => format!("{:11}", ""),
            }),
            MoveTo(25, 21),
            Print(match self.trades.last() {
                Some(trade) => format!("{:>12}", trade.qty),
                None => format!("{:12}", ""),
            }),
        )
        .unwrap();

        out.flush().unwrap();
    }
}
