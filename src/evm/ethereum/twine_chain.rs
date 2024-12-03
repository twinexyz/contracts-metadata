use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    contract TwineChain {

        #[derive(Debug)]
        struct TransactionObject {
            uint256 chainId;
            uint256 nonce;
            uint256 maxPriorityFeePerGas;
            uint256 maxFeePerGas;
            uint256 gas;
            address to;
            uint256 value;
            bytes input;
            AccessList[] accesslist;
            uint64 v;
            bytes32 r;
            bytes32 s;
        }

        #[derive(Debug)]
        struct AccessList {
            address _address;
            bytes32[] storageKeys;
        }


        #[derive(Debug)]
        struct CommitBatchInfo{
            uint64 batchNumber;
            bytes32 batchHash;
            bytes32 previousStateRoot;
            bytes32 stateRoot;
            bytes32 transactionRoot;
            bytes32 receiptRoot;
            TransactionObject[] depositTransactionObject;
            TransactionObject[] forcedTransactionObjects;
            TransactionObject[] otherTransactions;
        }

        #[derive(Debug)]
        function commitBatch(CommitBatchInfo calldata _newBatchData) external;

        #[derive(Debug)]
        function finalizeBatch(uint256 batchNumber, bytes calldata _proofBytes) external;
    }
}