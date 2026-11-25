use std::io::{Stdout, Write, stdout};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use oms::master::KrxSpotMaster;
use oms::quote::spot::{SpotFill, SpotQuoteWithoutLp};
use thousands::Separable;

use crate::dump_reader::DumpReader;

pub struct Display {
    path: String,
    master: KrxSpotMaster,
    delay: u64,
    out: Stdout,
}

impl Display {
    const Y_START: u16 = 4;

    pub fn new(path: &str, master: KrxSpotMaster, delay: u64) -> Result<Self> {
        Ok(Self {
            path: path.to_string(),
            master,
            delay,
            out: stdout(),
        })
    }

    pub fn show(&mut self) -> Result<()> {
        let reader = DumpReader::new(self.path.as_str())?;
        execute!(self.out, EnterAlternateScreen, Clear(ClearType::All), Hide)?;

        self.draw_master()?;

        reader.filter_map(|x| x.ok()).for_each(|x| {
            match &x.packet.identifier().data_code {
                b"B6" => {
                    let packet = x.packet.as_type::<SpotQuoteWithoutLp>();
                    if packet.instrument_code == self.master.종목코드 {
                        let _ = self.draw_order_book(packet);
                    } else {
                        return;
                    }
                }
                b"A3" => {
                    let packet = x.packet.as_type::<SpotFill>();
                    if packet.instrument_code == self.master.종목코드 {
                        let _ = self.draw_trade(packet);
                    } else {
                        return;
                    }
                }
                _ => return,
            }
            let _ = self.out.flush();

            thread::sleep(Duration::from_millis(self.delay));
        });

        execute!(self.out, Show, LeaveAlternateScreen)?;

        Ok(())
    }

    fn draw_master(&mut self) -> Result<()> {
        queue!(
            self.out,
            MoveTo(0, 0),
            Print(format!(
                "{}({})",
                self.master.종목한글약명, self.master.종목코드
            )),
            MoveTo(40, 0),
            Print(format!(
                "{:>10}",
                self.master.영업일자.to_date().format("%Y-%m-%d")
            )),
            MoveTo(0, 1),
            Print("전일누적거래량"),
            MoveTo(38, 1),
            Print(format!(
                "{:>12}",
                self.master.전일누적거래량.parse().separate_with_commas()
            )),
            MoveTo(0, 2),
            Print("전일누적거래대금"),
            MoveTo(31, 2),
            Print(format!(
                "{:>19}",
                self.master
                    .전일누적거래대금
                    .parse_integer_part()
                    .separate_with_commas()
            )),
            MoveTo(34, 4),
            Print(format!(
                "{:>11} 기준",
                self.master.기준가격.parse().separate_with_commas()
            )),
            MoveTo(34, 5),
            Print(
                format!(
                    "{:>11} 상한",
                    self.master.상한가.parse().separate_with_commas()
                )
                .red()
            ),
            MoveTo(34, 6),
            Print(
                format!(
                    "{:>11} 하한",
                    self.master.하한가.parse().separate_with_commas()
                )
                .blue()
            ),
        )?;
        Ok(())
    }

    fn draw_order_book(&mut self, packet: &SpotQuoteWithoutLp) -> Result<()> {
        for (i, quote) in packet.order_book.iter().enumerate() {
            let ask_y = Self::Y_START + 9 - i as u16;
            let ask_price = quote.prices.ask.parse();
            let ask_qty = quote.quantities.ask.parse();

            if quote.prices.ask.parse() != 0 {
                queue!(
                    self.out,
                    MoveTo(0, ask_y),
                    Print(format!("{:<16}", ask_qty.separate_with_commas())),
                    MoveTo(17, ask_y),
                    Print(format!("{:^16}", ask_price.separate_with_commas()).red()),
                )?;
            }

            let bid_y = Self::Y_START + 10 + i as u16;
            let bid_price = quote.prices.bid.parse();
            let bid_qty = quote.quantities.bid.parse();

            if bid_price != 0 {
                queue!(
                    self.out,
                    MoveTo(17, bid_y),
                    Print(format!("{:^16}", bid_price.separate_with_commas()).blue()),
                    MoveTo(34, bid_y),
                    Print(format!("{:>16}", bid_qty.separate_with_commas())),
                )?;
            }
        }
        queue!(
            self.out,
            MoveTo(17, Self::Y_START + 20),
            Print(format!(
                "{:^16}",
                packet.time.parse_time().format("%H:%M:%S")
            ))
        )?;
        Ok(())
    }

    fn draw_trade(&mut self, packet: &SpotFill) -> Result<()> {
        let y = Self::Y_START + 22;
        queue!(
            self.out,
            MoveTo(0, y),
            Print(format!(
                "{:<16}",
                packet.time.parse_time().format("%H:%M:%S")
            )),
            MoveTo(17, y),
            Print(format!(
                "{:^16}",
                packet.last_price.parse().separate_with_commas()
            )),
            MoveTo(34, y),
            Print(format!(
                "{:>16}",
                packet.volume.parse().separate_with_commas()
            )),
        )?;
        Ok(())
    }
}
