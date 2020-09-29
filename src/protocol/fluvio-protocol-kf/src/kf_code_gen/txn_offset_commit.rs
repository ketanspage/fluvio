/// WARNING: CODE GENERATED FILE
/// * This file is generated by kfspec2code.
/// * Any changes applied to this file will be lost when a new spec is generated.
use serde::{Deserialize, Serialize};

use kf_protocol_api::ErrorCode;
use kf_protocol_api::Request;

use kf_protocol_derive::Decode;
use kf_protocol_derive::Encode;
use kf_protocol_derive::KfDefault;

// -----------------------------------
// KfTxnOffsetCommitRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfTxnOffsetCommitRequest {
    /// The ID of the transaction.
    pub transactional_id: String,

    /// The ID of the group.
    pub group_id: String,

    /// The current producer ID in use by the transactional ID.
    pub producer_id: i64,

    /// The current epoch associated with the producer ID.
    pub producer_epoch: i16,

    /// Each topic that we want to committ offsets for.
    pub topics: Vec<TxnOffsetCommitRequestTopic>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct TxnOffsetCommitRequestTopic {
    /// The topic name.
    pub name: String,

    /// The partitions inside the topic that we want to committ offsets for.
    pub partitions: Vec<TxnOffsetCommitRequestPartition>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct TxnOffsetCommitRequestPartition {
    /// The index of the partition within the topic.
    pub partition_index: i32,

    /// The message offset to be committed.
    pub committed_offset: i64,

    /// The leader epoch of the last consumed record.
    #[fluvio_kf(min_version = 2, ignorable)]
    pub committed_leader_epoch: i32,

    /// Any associated metadata the client wants to keep.
    pub committed_metadata: Option<String>,
}

// -----------------------------------
// KfTxnOffsetCommitResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfTxnOffsetCommitResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation,
    /// or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The responses for each topic.
    pub topics: Vec<TxnOffsetCommitResponseTopic>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct TxnOffsetCommitResponseTopic {
    /// The topic name.
    pub name: String,

    /// The responses for each partition in the topic.
    pub partitions: Vec<TxnOffsetCommitResponsePartition>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct TxnOffsetCommitResponsePartition {
    /// The partitition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: ErrorCode,
}

// -----------------------------------
// Implementation - KfTxnOffsetCommitRequest
// -----------------------------------

impl Request for KfTxnOffsetCommitRequest {
    const API_KEY: u16 = 28;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 2;
    const DEFAULT_API_VERSION: i16 = 2;

    type Response = KfTxnOffsetCommitResponse;
}