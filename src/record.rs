use chrono::NaiveTime;
use oms::quote::packet::PacketBuffer;

pub struct Record {
    pub size: usize,
    pub time: NaiveTime,
    pub packet: PacketBuffer,
}

impl Record {
    pub const SIZE_LEN: usize = 4;
    pub const TIME_LEN: usize = 12;
}
