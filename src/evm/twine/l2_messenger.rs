use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    contract L2Messenger {

        // TODO: Remove height and root fields

        #[derive(Debug)]
        function verifyConsensusProofAndExecuteDeposit(
            uint256 chainId,
            uint256 height, // height or slot
            bytes32 root, // receipt_root or bank_hash
            bytes memory consensusProof,
            bytes memory depositTransactions,
            bytes32 parityHash
        );

        #[derive(Debug)]
        function executeForcedWithdrawal(
            uint256 chainId,
            uint256 height, // height or slot
            bytes32 root, // receipt_root or bank_hash
            bytes memory withdrawalTransaction,
            bytes32 parityHash
        );

        #[derive(Debug)]
        function verifyLayerZeroPayload(
            uint256 chainId,
            bytes memory lzPayload,
            bytes memory payloadProof
        );

        #[derive(Debug)]
        event ParityHash(
            bytes32 parityHash,
            uint256 blockNumber,
            bytes32 blockHash
        );

        #[derive(Debug)]
        event L1TokenDeposit();

        #[derive(Debug)]
        event ForcedWithdrawal(
            address l2Token,
            uint256 amount,
            uint256 l1Nonce,
            uint256 indexed chainId,
            uint256 blockNumber,
            uint256 gasLimit,
            string l1Token,
            string indexed from,
            string indexed to
        );


        #[derive(Debug)]
        event LayerzeroPayload(
            uint256 indexed sourceChainId,
            bytes32 indexed guId
        );

        #[derive(Debug)]
        event SentMessage(
            address indexed from,
            address l2Token,
            string to,
            string l1Token,
            uint256 amount,
            uint256 nonce,
            uint256 indexed chainId,
            uint256 blockNumber,
            uint256 gasLimit
        );

        #[derive(Debug)]
        struct WithdrawalDetails {
            uint256 l1Nonce;
            uint256 amount;
            address l2Token;
            string l1Token;
            string from;
            string to;      
        }
    }
}
