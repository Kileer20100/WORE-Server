use serde_json::{self, Error};
use thiserror::Error;
use tokio_tungstenite::tungstenite;


#[derive(Debug, Error)]
pub enum ServeError{
    #[error("Error send message: {0}")]
    ErrorSendMessage(String),
}
