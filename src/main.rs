use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use orderbook_cli::dump_reader::DumpReader;
use orderbook_cli::{display::Display, message::MessageKind};
use std::{io::stdout, thread, time::Duration};

fn main() -> Result<()> {
    let reader = DumpReader::new("krx_stock.20260904")?;
    let mut dis = Display::default();

    let mut out = stdout();
    execute!(out, EnterAlternateScreen, Clear(ClearType::All), Hide)?;
    reader
        .filter_map(|x| x.ok())
        .filter(|x| match &x.message.kind {
            MessageKind::Orderbook(msg) => msg.isin == "KR7005930003",
            MessageKind::Trade(msg) => msg.isin == "KR7005930003",
            _ => false,
        })
        .for_each(|x| {
            dis.update(x.message);
            dis.show();

            thread::sleep(Duration::from_millis(1));
        });
    execute!(out, Show, LeaveAlternateScreen)?;

    Ok(())
}
