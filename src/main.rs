use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use orderbook_cli::dump_reader::DumpReader;
use orderbook_cli::{client::Client, message::MessageKind};
use std::{
    io::{Error, stdout},
    thread,
    time::Duration,
};

fn main() -> Result<(), Error> {
    let reader = DumpReader::new("krx_stock.20260904")?;
    let mut client = Client::new();

    let mut out = stdout();
    execute!(out, EnterAlternateScreen, Clear(ClearType::All), Hide)?;
    reader
        .filter(|x| match &x.message.kind {
            MessageKind::Orderbook(msg) => msg.isin == "KR7005930003",
            MessageKind::Trade(msg) => msg.isin == "KR7005930003",
            _ => false,
        })
        .for_each(|x| {
            client.update(x.message);
            client.show();

            thread::sleep(Duration::from_millis(10));
        });
    execute!(out, Show, LeaveAlternateScreen)?;

    Ok(())
}
