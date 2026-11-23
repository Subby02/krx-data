#[derive(Debug, Default)]
pub struct Level {
    pub price: [u8; 11],
    pub qty: [u8; 12],
}

#[derive(Debug, Default)]
pub struct Orderbook {
    pub asks: [Level; 10],
    pub bids: [Level; 10],
}
