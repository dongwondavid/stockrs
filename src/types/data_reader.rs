use crate::types::trading::AssetInfo;
use crate::api::koreainvestapi::get_domestic006_result;
use crate::types::api::Domestic006Result;

pub enum DataReaderType {
    DB,
    PAPER,
    REAL,
}

pub trait DataReader {
    fn get_asset_info(&self) -> Result<AssetInfo, Box<dyn std::error::Error>>;
    fn get_avg_price(&self, stockcode: String) -> Result<f64, Box<dyn std::error::Error>>;
}

pub struct RealDataReader;
impl DataReader for RealDataReader {
    fn get_asset_info(&self) -> Result<AssetInfo, Box<dyn std::error::Error>> {
        let real: Domestic006Result = get_domestic006_result()?;
        Ok(real.into())
    }
    fn get_avg_price(&self, stockcode: String) -> Result<f64, Box<dyn std::error::Error>> {
        let result: Domestic006Result = get_domestic006_result()?;
        let avg = result.get_pchs_avg_pric(stockcode)?;
        Ok(avg)
    }
}

pub struct PaperDataReader;
impl DataReader for PaperDataReader {
    fn get_asset_info(&self) -> Result<AssetInfo, Box<dyn std::error::Error>> {
        todo!("get asset info");
    }
    fn get_avg_price(&self, stockcode: String) -> Result<f64, Box<dyn std::error::Error>> {
        todo!("get avg price");
    }
}

pub struct DbDataReader;
impl DataReader for DbDataReader {
    fn get_asset_info(&self) -> Result<AssetInfo, Box<dyn std::error::Error>> {
        todo!("get asset info");
    }
    fn get_avg_price(&self, stockcode: String) -> Result<f64, Box<dyn std::error::Error>> {
        todo!("get avg price");
    }
}

pub fn make_data_reader(
    kind: DataReaderType,
) -> Box<dyn DataReader> {
    match kind {
        DataReaderType::REAL => Box::new(RealDataReader),
        DataReaderType::DB => Box::new(DbDataReader),
        DataReaderType::PAPER => Box::new(PaperDataReader),
    }
} 