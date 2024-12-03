use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    contract L2Messenger {

        #[derive(Debug)]
        function verifyConsensusProofAndExecuteDeposit(
            uint256 chainId,
            bytes memory consensusProof,
            bytes[] memory depositTransactions,
            bytes[] memory depositTxnProofs,
        );

        #[derive(Debug)]
        function executeForcedWithdrawal(
            uint256 chainId,
            bytes memory withdrawalTransaction,
            bytes memory proof
        );

        #[derive(Debug)]
        function verifyLayerZeroPayload(
            uint256 chainId,
            bytes memory lzPayload,
            bytes memory payloadProof
        );
        

        #[derive(Debug)]
        event L1TokenDeposit();

        #[derive(Debug)]
        event ForcedWithdrawal(
            address indexed from,
            address to,
            address counterpartGateway,
            address counterpartMessenger,
            uint256 value,
            uint256 indexed chainId,
            uint256 blockNumber,
            uint256 gasLimit,
            bytes message
        );

        #[derive(Debug)]
        event LayerzeroPayload(
            uint256 indexed sourceChainId,
            bytes32 indexed guId
        );
    }
}
