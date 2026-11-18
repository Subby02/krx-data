#[derive(Debug, Default)]
pub struct Level {
    pub price: u64,
    pub qty: u64,
}

#[derive(Debug, Default)]
pub struct Orderbook {
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

impl Orderbook {
    pub fn insert_bid(&mut self, price: u64, qty: u64) {
        self.bids.push(Level { price, qty });
    }

    pub fn insert_ask(&mut self, price: u64, qty: u64) {
        self.asks.push(Level { price, qty });
    }
}
