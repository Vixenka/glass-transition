use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum NetworkError {
    UnableBindSocket,
    UnableCreateServerTransport,
    UnableCreateClientTransport,
    MissingAddress,
    InvalidAddress,
    InvalidPort,
}

impl Error for NetworkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NetworkError::MissingAddress => "Missing IP address",
                NetworkError::InvalidAddress => "Unable to parse IP address",
                NetworkError::InvalidPort =>
                    "Invalid port, expected decimal number from 0 to 65535",
                NetworkError::UnableBindSocket => "Unable to bind socket",
                NetworkError::UnableCreateServerTransport => "Unable to create server transport",
                NetworkError::UnableCreateClientTransport => "Unable to create client transport",
            }
        )
    }
}
