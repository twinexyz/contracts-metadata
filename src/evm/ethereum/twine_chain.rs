use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    contract TwineChain {

        #[derive(Debug)]
        struct ChainCommitment {
            uint64 depositCount;
            bytes32 depositRollingHash;
            uint64 withdrawCount;
            bytes32 withdrawRollingHash;
            uint64 lzTransactionCount;
            bytes32 lzTransactionRollingHash;
        }

        #[derive(Debug)]
        struct TransactionInfo {
            uint64 batchNumber;
            bytes32 transactionRoot;
        }

        #[derive(Debug)]
        struct StoredBatchInfo {
            uint64 batchNumber;
            bytes32 batchHash;
            bytes32 previousStateRoot;
            bytes32 stateRoot;
            bytes32 transactionRoot;
            bytes32 receiptRoot;
        }

        #[derive(Debug)]
        struct FinalizeWithdrawalInput {
            WithdrawalPublicInput publicInput;
            bytes inclusionProof;
        }

        #[derive(Debug)]
        struct WithdrawalPublicInput {
            bytes32 receiptRoot;
            uint64 chainId;
            uint64 batchNumber;
            uint64 nonce;
            uint8 isForced;
            string l1ReceiverAddress;
            string l1TokenAddress;
            string l2TokenAddress;
            string amount;
        }

        #[derive(Debug)]
        function commitAndFinalizeBatch(StoredBatchInfo memory commit_info, bytes memory execution_proof) external;

        #[derive(Debug)]
        function commitAndFinalizeTransactions(bytes memory transaction_info, bytes memory inclusion_proof) external;

        #[derive(Debug)]
        function finalizeWithdrawal(FinalizeWithdrawalInput memory withdrawalInputs) external;
    }
}
