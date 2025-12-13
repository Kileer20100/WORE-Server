
use thiserror::Error;



#[derive(Debug, Error)]
pub enum ServeError{
    /// Failed to send a message through the WebSocket connection.
    /// 
    /// This error occurs when the server cannot deliver a message to the client,
    /// typically due to a broken connection or network issues.
    /// 
    /// # Parameters
    /// - `0`: The underlying error message from the transport layer
    #[error("Error send message: {0}")]
    ErrorSendMessage(String),

    /// Failed to parse incoming WebSocket message as valid JSON.
    /// 
    /// Occurs when the client sends malformed or invalid JSON data
    /// that cannot be deserialized into the expected message structure.
    #[error("Invalid JSON format in WebSocket message")]
    ErrorRoutingParsingMessage,

    /// Received an unknown or unsupported action type.
    /// 
    /// The client requested an action that is not implemented
    /// or not recognized by the server's routing table.
    /// 
    /// # Parameters  
    /// - `0`: The action string that was requested
    #[error("Unknown action: '{0}'")]
    ErrorRoutingMessage(String),
}
