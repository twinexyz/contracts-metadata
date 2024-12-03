use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    contract L1MessageQueue{

        #[derive(Debug)]
        event QueueDepositTransaction(
            address indexed from,
            address to,
            uint256 value,
            uint256 indexed chainId,
            uint256 depositMessageIndex,
            uint256 gasLimit,
            uint256 blockNumber,
            bytes data
        );

        #[derive(Debug)]
        event QueueWithdrawalTransaction(
            address indexed from,
            address to,
            uint256 value,
            uint256 chainId,
            uint256 withdrawalMessageIndex,
            uint256 gasLimit,
            uint256 blockNumber,
            bytes data
        );
    }
}
