use std::{error::Error, fmt};

#[derive(Debug)]
pub enum DecodeError {
    InvalidLengthError,
    InvalidForcedFlag,
    InvalidL1ReceiverAddress,
    InvalidL1TokenAddress,
    InvalidL2TokenAddress,
    InvalidAmount,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeError::InvalidLengthError => {
                write!(f, "Invalid data length")
            }
            DecodeError::InvalidForcedFlag => {
                write!(f, "Invalid forced txn flag")
            }
            DecodeError::InvalidL1ReceiverAddress => {
                write!(f, "Invalid UTF-8 in l1_receiver_address")
            }
            DecodeError::InvalidL1TokenAddress => {
                write!(f, "Invalid UTF-8 in l1_token_address")
            }
            DecodeError::InvalidL2TokenAddress => {
                write!(f, "Invalid UTF-8 in l2_token_address")
            }
            DecodeError::InvalidAmount => {
                write!(f, "Invalid UTF-8 in amount")
            }
        }
    }
}

impl Error for DecodeError {}