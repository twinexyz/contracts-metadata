use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    contract L2Messenger {

        // TODO: Remove height and root fields

        #[derive(Debug)]
        function handleSolanaTransactions(
            uint256 chainId,
            bytes calldata precompileInput
        ); 

        #[derive(Debug)]
        function handleEthereumProofAndTransactions(
            uint256 chainId,
            bytes memory consensusProof,
            bytes memory ethereumTransactions
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
            uint256 value,
            uint256 nonce,
            uint256 indexed chainId,
            uint256 blockNumber,
            uint256 gasLimit
        );

        event solanaTransactionsHandled(bytes transactionOutput);
        event ethereumTransactionsHandled(bytes transactionOutput);

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

sol!{
    #[derive(Debug)]
    struct TxnDetail {
        uint8 status;
        uint64 chain_id;
        uint64 slot_number;
        string from_address;
        string to_twine_address;
        string l1_token;
        string l2_token;
        string amount;
    }

    #[derive(Debug)]
    struct DepositTxn {
        uint64 l1_nonce;
        TxnDetail detail;
    }

    #[derive(Debug)]
    struct WithdrawTxn {
        uint64 l1_nonce;
        TxnDetail detail;
    }

    #[derive(Debug)]
    struct PrecompileReturn {
        DepositTxn[] deposit;
        WithdrawTxn[] withdraws;
    }
}
