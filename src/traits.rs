use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub struct Trading {
    date: NaiveDateTime,
    stockcode: [u8; 6],
    buy_or_sell: bool,
    quantity: u32,
    price: f64,
    fee: f64,
    strategy: String,
}

pub struct TradingResult {
    date: NaiveDate,
    time: NaiveTime,
    stockcode: [u8; 6],
    buy_or_sell: bool,
    quantity: u32,
    price: f64,
    fee: f64,
    strategy: String,
    avg_price: f64,
    profit: f64,
    roi: f64,
}

struct Domestic006Output1 {
    pdno: String,
    pchs_avg_pric: String,
}

struct Domestic006Output2 {
    dnca_tot_amt: String,
    nass_amt: String,
}

pub struct Domestic006Result {
    date: NaiveDateTime,
    output1: Vec<Domestic006Output1>,
    output2: Domestic006Output2,
}

impl Domestic006Result {
    fn new(date: NaiveDateTime, output1: Vec<Domestic006Output1>, output2: Domestic006Output2) -> Self {
        Self { date, output1, output2 }
    }

    pub fn get_date(&self) -> NaiveDateTime { self.date }

    pub fn get_asset(&self) -> f64 { self.output2.nass_amt.parse::<f64>().unwrap() }
}

impl Trading {
    fn new(date: NaiveDateTime, stockcode: [u8; 6], buy_or_sell: bool, quantity: u32, price: f64, fee: f64, strategy: String) -> Self {
        Self { date, stockcode, buy_or_sell, quantity, price, fee, strategy }
    }

    pub fn to_trading_result(&self) -> TradingResult {

        let avg_price = todo!("get avg price");
        let profit = match self.buy_or_sell {
            true => -self.fee as f64, // 매수면 손실금 = 수수료 
            false => (self.price - avg_price) * self.quantity as f64 - self.fee as f64, // 매도면 손익금 = (매도가 - 평균매입가) * 수량 - 수수료
        };

        let roi = profit / (avg_price * self.quantity as f64) * 100.0;

        TradingResult::new(self.date.date(), self.date.time(), self.stockcode, self.buy_or_sell, self.quantity, self.price, self.fee, self.strategy, avg_price, profit, roi)

    }
}

impl TradingResult {
    fn new(date: NaiveDate, time: NaiveTime, stockcode: [u8; 6], buy_or_sell: bool, quantity: u32, price: f64, fee: f64, strategy: String, avg_price: f64, profit: f64, roi: f64) -> Self {
        Self { date, time, stockcode, buy_or_sell, quantity, price, fee, strategy, avg_price, profit, roi }
    }
}
