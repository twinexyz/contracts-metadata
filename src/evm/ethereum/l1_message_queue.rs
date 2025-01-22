use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    contract L1MessageQueue{

        #[derive(Debug)]
        event QueueDepositTransaction(
            uint64 nonce,
            uint64 chainId,
            uint64 block_number,
            address l1_token,
            address l2_token,
            address from,
            address to_twine_address,
            uint256 amount,
        );

        #[derive(Debug)]
        event QueueWithdrawalTransaction(
            uint64 nonce,
            uint64 chainId,
            uint64 block_number,
            address l1_token,
            address l2_token,
            address from,
            address to_twine_address,
            uint256 amount,
        );
    }
}
