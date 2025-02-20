use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    contract L1MessageQueue{

        #[derive(Debug)]
        struct MessageData {
            uint64 nonce;
            uint64 chainId;
            uint64 blockNumber;
            string fromAddress;
            string toAddress;
            string l1Token;
            string l2Token;
            string amount;
        }

        #[derive(Debug)]
        event QueueDepositTransaction(
            uint64 nonce,
            uint64 chainId,
            uint64 blockNumber,
            address l1Token,
            address l2Token,
            address from,
            address toTwineAddress,
            uint256 amount
        );

        #[derive(Debug)]
        event QueueWithdrawalTransaction(
            uint64 nonce,
            uint64 chainId,
            uint64 blockNumber,
            address l1Token,
            address l2Token,
            address from,
            address toTwineAddress,
            uint256 amount
        );
    }
}
