use anyhow::{Result, bail};
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use orderbook_cli::{display::Display, message::Message};
use orderbook_cli::{dump_reader::DumpReader, message::MessageBody};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, stdout},
    thread,
    time::Duration,
};

fn get_display(isin: &String) -> Result<Display> {
    let mut master_reader = BufReader::new(File::open("krxstk_mst.20260904")?);
    let mut buf = Vec::new();

    while master_reader.read_until(b'\n', &mut buf)? > 0 {
        let line = buf.strip_suffix(b"\n").unwrap_or(&buf);
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        let master_message = Message::new(&line.to_vec())?;
        if let MessageBody::A0(body) = &master_message.body
            && body.isin == isin.as_bytes()
        {
            return Display::new(&master_message);
        }
        buf.clear();
    }

    bail!("마스터 파일에서 종목을 찾지 못함: {isin}")
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let isin = &args[1];
    let delay = &args[2].parse::<u64>()?;
    let mut dis = get_display(isin)?;

    let reader = DumpReader::new("krx_stock.20260904")?;

    let mut out = stdout();
    execute!(out, EnterAlternateScreen, Clear(ClearType::All), Hide)?;
    reader
        .filter_map(|x| x.ok())
        .filter(|x| match &x.message.body {
            MessageBody::B6(body) => body.isin == isin.as_bytes(),
            MessageBody::A3(body) => body.isin == isin.as_bytes(),
            _ => false,
        })
        .for_each(|x| {
            let _ = dis.update(x.message);
            let _ = dis.show();

            thread::sleep(Duration::from_millis(*delay));
        });
    execute!(out, Show, LeaveAlternateScreen)?;

    Ok(())
}
