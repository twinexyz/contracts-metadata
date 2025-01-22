use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    contract L1MessageQueue{

        #[derive(Debug)]
        event QueueDepositTransaction(
            uint64 nonce,
            string to_twine_address,
            string l1_token,
            string l2_token,
            uint64 chainId,
            string amount,
            uint64 block_number
        );

        #[derive(Debug)]
        event QueueWithdrawalTransaction(
            uint64 nonce,
            string to_twine_address,
            string l1_token,
            string l2_token,
            uint64 chainId,
            string amount,
            uint64 block_number
        );
    }
}
