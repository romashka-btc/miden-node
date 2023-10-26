#[derive(Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MmrDelta {
    #[prost(uint64, tag = "1")]
    pub forest: u64,
    #[prost(message, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<super::digest::Digest>,
}
