use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    contract TwineDVN{
        #[derive(Debug)]
        event TwineNotified(
            uint32 indexed dstEid,
            uint64 blockConfirmations,
            address receiverAddress,
            uint256 fee,
            uint256 blockNumber,
            bytes32 payloadHash,
            bytes32 guId,
            bytes packetHeader
        );
    }
}
