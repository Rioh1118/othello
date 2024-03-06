use thiserror::Error;

#[derive(Error,Debug)]
pub enum OthelloError {
    #[error("Beyond the 8x8 range. It's {0} {1}")]
    OutOfRange(u8,u8),
    #[error("invalid move")]
    InvalidMove,
}