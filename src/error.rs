use thiserror::Error;

#[derive(Error, Debug)]
pub enum StockpileError {
    #[error("I'm too lazy to write errors")]
    DefaultError
}