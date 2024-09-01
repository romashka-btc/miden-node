// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ApplyBlockResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNullifiersResponse {
    /// Each requested nullifier has its corresponding nullifier proof at the same position.
    #[prost(message, repeated, tag = "1")]
    pub proofs: ::prost::alloc::vec::Vec<super::smt::SmtOpening>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckNullifiersByPrefixResponse {
    /// List of nullifiers matching the prefixes specified in the request.
    #[prost(message, repeated, tag = "1")]
    pub nullifiers: ::prost::alloc::vec::Vec<NullifierUpdate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockHeaderByNumberResponse {
    /// The requested block header
    #[prost(message, optional, tag = "1")]
    pub block_header: ::core::option::Option<super::block::BlockHeader>,
    /// Merkle path to verify the block's inclusion in the MMR at the returned `chain_length`
    #[prost(message, optional, tag = "2")]
    pub mmr_path: ::core::option::Option<super::merkle::MerklePath>,
    /// Current chain length
    #[prost(fixed32, optional, tag = "3")]
    pub chain_length: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct NullifierUpdate {
    #[prost(message, optional, tag = "1")]
    pub nullifier: ::core::option::Option<super::digest::Digest>,
    #[prost(fixed32, tag = "2")]
    pub block_num: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncStateResponse {
    /// Number of the latest block in the chain
    #[prost(fixed32, tag = "1")]
    pub chain_tip: u32,
    /// Block header of the block with the first note matching the specified criteria
    #[prost(message, optional, tag = "2")]
    pub block_header: ::core::option::Option<super::block::BlockHeader>,
    /// Data needed to update the partial MMR from `request.block_num + 1` to `response.block_header.block_num`
    #[prost(message, optional, tag = "3")]
    pub mmr_delta: ::core::option::Option<super::mmr::MmrDelta>,
    /// List of account hashes updated after `request.block_num + 1` but not after `response.block_header.block_num`
    #[prost(message, repeated, tag = "5")]
    pub accounts: ::prost::alloc::vec::Vec<super::account::AccountSummary>,
    /// List of transactions executed against requested accounts between `request.block_num + 1` and
    /// `response.block_header.block_num`
    #[prost(message, repeated, tag = "6")]
    pub transactions: ::prost::alloc::vec::Vec<super::transaction::TransactionSummary>,
    /// List of all notes together with the Merkle paths from `response.block_header.note_root`
    #[prost(message, repeated, tag = "7")]
    pub notes: ::prost::alloc::vec::Vec<super::note::NoteSyncRecord>,
    /// List of nullifiers created between `request.block_num + 1` and `response.block_header.block_num`
    #[prost(message, repeated, tag = "8")]
    pub nullifiers: ::prost::alloc::vec::Vec<NullifierUpdate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncNoteResponse {
    /// Number of the latest block in the chain
    #[prost(fixed32, tag = "1")]
    pub chain_tip: u32,
    /// Block header of the block with the first note matching the specified criteria
    #[prost(message, optional, tag = "2")]
    pub block_header: ::core::option::Option<super::block::BlockHeader>,
    /// Merkle path to verify the block's inclusion in the MMR at the returned `chain_tip`.
    ///
    /// An MMR proof can be constructed for the leaf of index `block_header.block_num` of
    /// an MMR of forest `chain_tip` with this path.
    #[prost(message, optional, tag = "3")]
    pub mmr_path: ::core::option::Option<super::merkle::MerklePath>,
    /// List of all notes together with the Merkle paths from `response.block_header.note_root`
    #[prost(message, repeated, tag = "4")]
    pub notes: ::prost::alloc::vec::Vec<super::note::NoteSyncRecord>,
}
/// An account returned as a response to the GetBlockInputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBlockInputRecord {
    #[prost(message, optional, tag = "1")]
    pub account_id: ::core::option::Option<super::account::AccountId>,
    #[prost(message, optional, tag = "2")]
    pub account_hash: ::core::option::Option<super::digest::Digest>,
    #[prost(message, optional, tag = "3")]
    pub proof: ::core::option::Option<super::merkle::MerklePath>,
}
/// A nullifier returned as a response to the GetBlockInputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullifierBlockInputRecord {
    #[prost(message, optional, tag = "1")]
    pub nullifier: ::core::option::Option<super::digest::Digest>,
    #[prost(message, optional, tag = "2")]
    pub opening: ::core::option::Option<super::smt::SmtOpening>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockInputsResponse {
    /// The latest block header
    #[prost(message, optional, tag = "1")]
    pub block_header: ::core::option::Option<super::block::BlockHeader>,
    /// Peaks of the above block's mmr, The `forest` value is equal to the block number
    #[prost(message, repeated, tag = "2")]
    pub mmr_peaks: ::prost::alloc::vec::Vec<super::digest::Digest>,
    /// The hashes of the requested accounts and their authentication paths
    #[prost(message, repeated, tag = "3")]
    pub account_states: ::prost::alloc::vec::Vec<AccountBlockInputRecord>,
    /// The requested nullifiers and their authentication paths
    #[prost(message, repeated, tag = "4")]
    pub nullifiers: ::prost::alloc::vec::Vec<NullifierBlockInputRecord>,
    /// The list of requested notes which were found in the database
    #[prost(message, optional, tag = "5")]
    pub found_unauthenticated_notes: ::core::option::Option<
        super::note::NoteAuthenticationInfo,
    >,
}
/// An account returned as a response to the GetTransactionInputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct AccountTransactionInputRecord {
    #[prost(message, optional, tag = "1")]
    pub account_id: ::core::option::Option<super::account::AccountId>,
    /// The latest account hash, zero hash if the account doesn't exist.
    #[prost(message, optional, tag = "2")]
    pub account_hash: ::core::option::Option<super::digest::Digest>,
}
/// A nullifier returned as a response to the GetTransactionInputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct NullifierTransactionInputRecord {
    #[prost(message, optional, tag = "1")]
    pub nullifier: ::core::option::Option<super::digest::Digest>,
    /// The block at which the nullifier has been consumed, zero if not consumed.
    #[prost(fixed32, tag = "2")]
    pub block_num: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionInputsResponse {
    #[prost(message, optional, tag = "1")]
    pub account_state: ::core::option::Option<AccountTransactionInputRecord>,
    #[prost(message, repeated, tag = "2")]
    pub nullifiers: ::prost::alloc::vec::Vec<NullifierTransactionInputRecord>,
    #[prost(message, repeated, tag = "3")]
    pub missing_unauthenticated_notes: ::prost::alloc::vec::Vec<super::digest::Digest>,
    #[prost(fixed32, tag = "4")]
    pub block_height: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SubmitProvenTransactionResponse {
    /// The node's current block height
    #[prost(fixed32, tag = "1")]
    pub block_height: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotesByIdResponse {
    /// Lists Note's returned by the database
    #[prost(message, repeated, tag = "1")]
    pub notes: ::prost::alloc::vec::Vec<super::note::Note>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNoteAuthenticationInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub proofs: ::core::option::Option<super::note::NoteAuthenticationInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNullifiersResponse {
    /// Lists all nullifiers of the current chain
    #[prost(message, repeated, tag = "1")]
    pub nullifiers: ::prost::alloc::vec::Vec<super::smt::SmtLeafEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountsResponse {
    /// Lists all accounts of the current chain
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<super::account::AccountInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotesResponse {
    /// Lists all notes of the current chain
    #[prost(message, repeated, tag = "1")]
    pub notes: ::prost::alloc::vec::Vec<super::note::Note>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountDetailsResponse {
    /// Account info (with details for on-chain accounts)
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<super::account::AccountInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByNumberResponse {
    /// The requested `Block` data encoded using miden native format
    #[prost(bytes = "vec", optional, tag = "1")]
    pub block: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountStateDeltaResponse {
    /// The calculated `AccountStateDelta` encoded using miden native format
    #[prost(bytes = "vec", optional, tag = "1")]
    pub delta: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
