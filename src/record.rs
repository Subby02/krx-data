use crate::message::Message;
use chrono::NaiveTime;

#[derive(Debug)]
pub struct Record {
    pub size: usize,
    pub time: NaiveTime,
    pub message: Message,
}

impl Record {
    pub const SIZE_LEN: usize = 4;
    pub const TIME_LEN: usize = 12;
}
