use std::io::{Write, stdout};

use anyhow::{Result, bail};
use chrono::{NaiveDate, NaiveTime};
use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Print, Stylize};
use encoding_rs::EUC_KR;
use rust_decimal::Decimal;
use thousands::Separable;

use crate::message::{Message, MessageBody};
use crate::orderbook::Orderbook;
use crate::utils::{to_decimal, to_str, to_u64};

struct Trade {
    time: NaiveTime,
    price: u64,
    qty: u64,
}

#[derive(Default)]
pub struct Display {
    name: String,
    code: String,
    date: NaiveDate,
    base_price: u64,
    upper_limit: u64,
    lower_limit: u64,
    prev_volume: u64,
    prev_amount: Decimal,
    time: NaiveTime,
    orderbook: Orderbook,
    trades: Vec<Trade>,
}

impl Display {
    pub fn new(message: &Message) -> Result<Self> {
        if let MessageBody::A0(body) = &message.body {
            return Ok(Self {
                name: EUC_KR.decode(&body.name_kr).0.trim().to_string(),
                code: to_str(&body.short_code)?,
                date: NaiveDate::parse_from_str(to_str(&body.biz_date)?.as_str(), "%Y%m%d")?,
                base_price: to_u64(&body.base_price)?,
                upper_limit: to_u64(&body.upper_limit)?,
                lower_limit: to_u64(&body.lower_limit)?,
                prev_volume: to_u64(&body.prev_volume)?,
                prev_amount: to_decimal(&body.prev_amount)?,
                ..Default::default()
            });
        }
        bail!("메세지를 생성 할 수 없습니다.");
    }

    pub fn update(&mut self, message: Message) -> Result<()> {
        match message.body {
            MessageBody::B6(body) => {
                self.time =
                    NaiveTime::parse_from_str(to_str(&body.proc_time)?.as_str(), "%H%M%S%6f")?;
                self.orderbook = body.orderbook;
            }
            MessageBody::A3(body) => {
                self.trades.push(Trade {
                    time: NaiveTime::parse_from_str(
                        to_str(&body.proc_time)?.as_str(),
                        "%H%M%S%6f",
                    )?,
                    price: to_u64(&body.trade_price)?,
                    qty: to_u64(&body.trade_qty)?,
                });
            }
            _ => {}
        }
        Ok(())
    }

    pub fn show(&self) -> Result<()> {
        let mut out = stdout();

        queue!(
            out,
            MoveTo(0, 0),
            Print(format!("{}({})", self.name, self.code)),
            MoveTo(40, 0),
            Print(format!("{:>10}", self.date.format("%Y-%m-%d"))),
            MoveTo(0, 1),
            Print("전일누적거래량"),
            MoveTo(38, 1),
            Print(format!("{:>12}", self.prev_volume.separate_with_commas())),
            MoveTo(0, 2),
            Print("전일누적거래대금"),
            MoveTo(31, 2),
            Print(format!(
                "{:>19}",
                self.prev_amount.trunc().separate_with_commas()
            )),
            MoveTo(34, 4),
            Print(format!(
                "{:>11} 기준",
                self.base_price.separate_with_commas()
            )),
            MoveTo(34, 5),
            Print(format!("{:>11} 상한", self.upper_limit.separate_with_commas()).red()),
            MoveTo(34, 6),
            Print(format!("{:>11} 하한", self.lower_limit.separate_with_commas()).blue()),
        )?;

        let orderbook_start_y = 4;
        for (i, ask) in self.orderbook.asks.iter().enumerate() {
            let y = orderbook_start_y + 9 - i as u16;
            let ask_price = to_u64(&ask.price)?;
            let ask_qty = to_u64(&ask.qty)?;

            if ask_price != 0 {
                queue!(
                    out,
                    MoveTo(0, y),
                    Print(format!("{:<16}", ask_qty.separate_with_commas())),
                    MoveTo(17, y),
                    Print(format!("{:^16}", ask_price.separate_with_commas()).blue()),
                )?;
            }
        }

        for (i, bid) in self.orderbook.bids.iter().enumerate() {
            let y = orderbook_start_y + 10 + i as u16;
            let bid_price = to_u64(&bid.price)?;
            let bid_qty = to_u64(&bid.qty)?;

            if bid_price != 0 {
                queue!(
                    out,
                    MoveTo(17, y),
                    Print(format!("{:^16}", bid_price.separate_with_commas()).red()),
                    MoveTo(34, y),
                    Print(format!("{:>16}", bid_qty.separate_with_commas())),
                    MoveTo(17, orderbook_start_y + 20),
                    Print(format!("{:^16}", self.time.format("%H:%M:%S"))),
                )?;
            }
        }

        for (i, trade) in self.trades[self.trades.len().saturating_sub(5)..]
            .iter()
            .enumerate()
        {
            let y = orderbook_start_y + 26 - i as u16;
            queue!(
                out,
                MoveTo(0, y),
                Print(format!("{:<16}", trade.time.format("%H:%M:%S"))),
                MoveTo(17, y),
                Print(format!("{:^16}", trade.price.separate_with_commas())),
                MoveTo(34, y),
                Print(format!("{:>16}", trade.qty.separate_with_commas())),
            )?;
        }

        out.flush()?;
        Ok(())
    }
}
