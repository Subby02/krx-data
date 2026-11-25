use anyhow::{Result, bail};
use oms::{
    master::{KrxSpotMaster, LoadableMaster},
    primitive::InstrumentCode,
};
use orderbook_cli::display::Display;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let isin: [u8; 12] = args[1].as_bytes().try_into().unwrap();
    let delay = &args[2].parse::<u64>()?;

    let krx_masters = KrxSpotMaster::load_masters("krxstk_mst.20260904")?;

    let master = krx_masters
        .iter()
        .find(|m| m.종목코드 == InstrumentCode::from(&isin));

    match master {
        Some(master) => {
            let mut display = Display::new("krx_stock.20260904", *master, *delay)?;

            display.show()?;
        }

        None => bail!("마스터 파일에서 종목을 찾지 못함"),
    }

    Ok(())
}
