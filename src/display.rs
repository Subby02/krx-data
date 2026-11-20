use std::io::{Write, stdout};

use chrono::NaiveTime;
use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, Stylize};

use crate::message::{Message, MessageBody};
use crate::orderbook::Orderbook;

struct Trade {
    time: NaiveTime,
    price: u64,
    qty: u64,
}

#[derive(Default)]
pub struct Display {
    time: NaiveTime,
    orderbook: Orderbook,
    trades: Vec<Trade>,
}

impl Display {
    pub fn update(&mut self, message: Message) {
        match message.body {
            MessageBody::B6(body) => {
                self.time =
                    NaiveTime::parse_from_str(body.proc_time.as_str(), "%H%M%S%6f").unwrap();
                self.orderbook = body.orderbook;
            }
            MessageBody::A3(body) => {
                self.trades.push(Trade {
                    time: NaiveTime::parse_from_str(body.proc_time.as_str(), "%H%M%S%6f").unwrap(),
                    price: body.trade_price,
                    qty: body.trade_qty,
                });
            }
            _ => {}
        }
    }

    pub fn show(&self) {
        let mut out = stdout();

        for (i, bid) in self.orderbook.bids.iter().enumerate() {
            let y = 10 - i as u16;
            if bid.price != 0 {
                queue!(
                    out,
                    MoveTo(0, 0),
                    Print(format!("{:<12}", self.time.format("%H:%M:%S"))),
                    MoveTo(0, y),
                    Print(format!("{:<12}", bid.qty)),
                    MoveTo(13, y),
                    Print(format!("{:^11}", bid.price).blue()),
                )
                .unwrap();
            }
        }

        for (i, ask) in self.orderbook.asks.iter().enumerate() {
            let y = 11 + i as u16;
            if ask.price != 0 {
                queue!(
                    out,
                    MoveTo(13, y),
                    Print(format!("{:^11}", ask.price).red()),
                    MoveTo(25, y),
                    Print(format!("{:>12}", ask.qty)),
                )
                .unwrap();
            }
        }

        for (i, trade) in self.trades[self.trades.len().saturating_sub(5)..]
            .iter()
            .enumerate()
        {
            let y = 26 - i as u16;
            queue!(
                out,
                MoveTo(0, y),
                Print(format!("{:<12}", trade.time.format("%H:%M:%S"))),
                MoveTo(13, y),
                Print(format!("{:^11}", trade.price)),
                MoveTo(25, y),
                Print(format!("{:>12}", trade.qty)),
            )
            .unwrap();
        }

        out.flush().unwrap();
    }
}
