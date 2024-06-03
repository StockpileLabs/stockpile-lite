use thiserror::Error;

#[derive(Error, Debug)]
pub enum StockpileError {
    #[error("I'm too lazy to write errors")]
    DefaultError,
    #[error("Pool is pending start")]
    PendingStart,
    #[error("Pool funds have been distributed")]
    PoolDistributed,
    #[error("This vault is closed")]
    VaultClosed,
    #[error("This vault is deactivated")]
    VaultDeactivated
}