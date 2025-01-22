use std::error::Error;

use super::{error::DecodeError, twine_chain::TwineChain::WithdrawalPublicInput};

impl WithdrawalPublicInput {
    pub fn abi_decode_packed_ethereum(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Self::abi_decode_packed(data, 42, 42)
    }

    pub fn abi_decode_packed_solana(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Self::abi_decode_packed(data, 44, 44)
    }

    pub fn abi_decode_packed(
        data: Vec<u8>,
        l1_receiver_address_len: usize,
        l1_token_address_len: usize,
    ) -> Result<Self, Box<dyn Error>> {
        // l2 address always is of length 42
        let l2_token_address_len: usize = 42;

        // Calculate the expected fixed portion length
        let fixed_len = 32 + 8 + 8 + 8 + 1; // receiptRoot + chainId + batchNumber + nonce + isForced
        let expected_len = fixed_len
            + l1_receiver_address_len
            + l1_token_address_len
            + l2_token_address_len
            + (data.len()
                - fixed_len
                - l1_receiver_address_len
                - l1_token_address_len
                - l2_token_address_len);

        if data.len() < expected_len {
            return Err(Box::new(DecodeError::InvalidLengthError));
        }

        // Decode fixed fields
        let receipt_root: [u8; 32] = data[0..32].try_into()?;
        let chain_id = u64::from_be_bytes(data[32..40].try_into()?);
        let batch_number = u64::from_be_bytes(data[40..48].try_into()?);
        let nonce = u64::from_be_bytes(data[48..56].try_into()?);
        let is_forced = data[56];

        // Decode dynamic fields
        let l1_receiver_address_start = 57;
        let l1_receiver_address = std::str::from_utf8(
            &data[l1_receiver_address_start..l1_receiver_address_start + l1_receiver_address_len],
        )
        .map_err(|_| DecodeError::InvalidL1ReceiverAddress)?
        .to_string();

        let l1_token_address_start = l1_receiver_address_start + l1_receiver_address_len;
        let l1_token_address = std::str::from_utf8(
            &data[l1_token_address_start..l1_token_address_start + l1_token_address_len],
        )
        .map_err(|_| DecodeError::InvalidL1TokenAddress)?
        .to_string();

        let l2_token_address_start = l1_token_address_start + l1_token_address_len;
        let l2_token_address = std::str::from_utf8(
            &data[l2_token_address_start..l2_token_address_start + l2_token_address_len],
        )
        .map_err(|_| DecodeError::InvalidL2TokenAddress)?
        .to_string();

        let amount_start = l2_token_address_start + l2_token_address_len;
        let amount = std::str::from_utf8(&data[amount_start..])
            .map_err(|_| DecodeError::InvalidAmount)?
            .to_string();

        Ok(Self {
            amount: amount,
            receiptRoot: alloy_sol_types::private::FixedBytes(receipt_root),
            chainId: chain_id,
            batchNumber: batch_number,
            nonce: nonce,
            isForced: is_forced,
            l1ReceiverAddress: l1_receiver_address,
            l1TokenAddress: l1_token_address,
            l2TokenAddress: l2_token_address,
        })
    }
}
