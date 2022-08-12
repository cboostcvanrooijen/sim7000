use crate::modem::at_command::{ATParseErr, ATParseLine};

#[derive(Debug, PartialEq, Eq)]
pub struct Connection {
    pub index: usize,
    pub message: ConnectionMessage,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionMessage {
    /// The connection was successfully established
    Connected,

    /// Failed to establish connection
    ConnectionFailed,

    /// A connection already exists on this index
    AlreadyConnected,

    /// A message was successfully sent
    SendSuccess,

    /// Failed to send message
    SendFail,

    /// The connection was closed
    Closed,
}

impl ATParseLine for Connection {
    fn from_line(line: &str) -> Result<Self, ATParseErr> {
        let (index, message) = line.split_once(", ").ok_or(ATParseErr)?;
        let index = index.parse()?;

        use ConnectionMessage::*;
        let message = match message {
            "CLOSED" | "CLOSE OK" => Closed,
            "SEND OK" => SendSuccess,
            "SEND FAIL" => SendFail,
            "CONNECT OK" => Connected,
            "CONNECT FAIL" => ConnectionFailed,
            "ALREADY CONNECT" => AlreadyConnected,
            _ => {
                return Err(ATParseErr);
            }
        };

        Ok(Connection { index, message })
    }
}
