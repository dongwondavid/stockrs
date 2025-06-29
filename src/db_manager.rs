use rusqlite::{Connection, Result};
use crate::traits::{Trading, TradingResult};
use std::path::PathBuf;
use crate::data_reader::get_account_info;

struct DBManager {
    conn: Connection,
}

impl DBManager {
    fn new(path: PathBuf) -> Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS trading (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT,
                time TEXT,
                stockcode TEXT,
                buy_or_sell TEXT,
                quantity INTEGER,
                price REAL,
                fee REAL,
                strategy TEXT,
                avg_price REAL,
                profit REAL,
                roi REAL
            )",
            (),
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS overview (
                date TEXT PRIMARY KEY,
                open REAL,
                high REAL,
                low REAL,
                close REAL,
                volume INTEGER,
                turnover REAL,
                profit REAL,
                roi REAL,
                fee REAL
            )",
            (),
        )?;

        Ok(Self { conn })
    }

    fn save_trading(&self, trading: Trading) -> Result<()> {
        let trading_result = trading.to_trading_result();
        todo!("save trading to db");
    }

    // Initialize today's overview data
    fn insert_overview(&self) -> Result<()> {
        let result = get_account_info().unwrap();
        let date = result.get_date();
        let asset = result.get_asset();

        self.conn.execute(
            "INSERT INTO overview (date, open, high, low) VALUES (?, ?, ?, ?)",
            (date.date().to_string(), asset, asset, asset),
        )?;

        Ok(())
    }

    // Update today's overview data
    fn update_overview(&self) -> Result<()> {
        let result = get_account_info().unwrap();
        let date = result.get_date();
        let asset = result.get_asset();

        // Get today's high and low values
        let (high, low) = self.conn.query_row(
            "SELECT high, low FROM overview WHERE date = ?",
            (date.date().to_string(),),
            |row| {
                let high: f64 = row.get(0)?;
                let low: f64 = row.get(1)?;
                Ok((high, low))
            },
        )?;

        // Update with new values
        let new_high = high.max(asset);
        let new_low = low.min(asset);

        self.conn.execute(
            "UPDATE overview SET high = ?, low = ? WHERE date = ?",
            (new_high, new_low, date.date().to_string()),
        )?;

        Ok(())
    }

    // Finalize today's overview data
    fn finish_overview(&self) -> Result<()> {
        let result = get_account_info().unwrap();
        let date = result.get_date();
        let asset = result.get_asset();

        let open: f64 = self.conn.query_row(
            "SELECT open FROM overview WHERE date = ?",
            (date.date().to_string(),),
            |row| row.get(0),
        )?;

        let close = asset;

        let daily_profit = close - open;
        let daily_roi = daily_profit / open * 100.0;

        // 오늘 날짜의 수수료, 총 거래대금 조회
        let fee_sum: Option<f64> = self.conn.query_row(
            "SELECT SUM(fee) FROM trading WHERE date = ?",
            (date.date().to_string(),),
            |row| row.get(0),
        )?;

        let turnover_sum: Option<f64> = self.conn.query_row(
            "SELECT SUM(price * quantity) FROM trading WHERE date = ?",
            (date.date().to_string(),),
            |row| row.get(0),
        )?;

        let fee = fee_sum.unwrap_or(0.0);
        let turnover = turnover_sum.unwrap_or(0.0);

        self.conn.execute(
            "UPDATE overview SET close = ?, profit = ?, roi = ?, fee = ?, turnover = ? WHERE date = ?",
            (close, daily_profit, daily_roi, fee, turnover, date.date().to_string()),
        )?;

        Ok(())
    }
}