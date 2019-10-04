use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt;

/// See [Kafka Errors](http://kafka.apache.org/protocol.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(i16)]
pub enum Error {
    /// The server experienced an unexpected error when processing the request
    Unknown = -1,
    None = 0,
    /// The requested offset is outside the range of offsets
    /// maintained by the server for the given topic/partition
    OffsetOutOfRange = 1,
    /// This indicates that a message contents does not match its CRC
    CorruptMessage = 2,
    /// This request is for a topic or partition that does not exist
    /// on this broker.
    UnknownTopicOrPartition = 3,
    /// The message has a negative size
    InvalidMessageSize = 4,
    /// This error is thrown if we are in the middle of a leadership
    /// election and there is currently no leader for this partition
    /// and hence it is unavailable for writes.
    LeaderNotAvailable = 5,
    /// This error is thrown if the client attempts to send messages
    /// to a replica that is not the leader for some partition. It
    /// indicates that the clients metadata is out of date.
    NotLeaderForPartition = 6,
    /// This error is thrown if the request exceeds the user-specified
    /// time limit in the request.
    RequestTimedOut = 7,
    /// This is not a client facing error and is used mostly by tools
    /// when a broker is not alive.
    BrokerNotAvailable = 8,
    /// If replica is expected on a broker, but is not (this can be
    /// safely ignored).
    ReplicaNotAvailable = 9,
    /// The server has a configurable maximum message size to avoid
    /// unbounded memory allocation. This error is thrown if the
    /// client attempt to produce a message larger than this maximum.
    MessageSizeTooLarge = 10,
    /// Internal error code for broker-to-broker communication.
    StaleControllerEpoch = 11,
    /// If you specify a string larger than configured maximum for
    /// offset metadata
    OffsetMetadataTooLarge = 12,
    /// The server disconnected before a response was received.
    NetworkException = 13,
    /// The coordinator is loading and hence can't process requests.
    CoordinatorLoadInProgress = 14,
    /// The coordinator is not available.
    CoordinatorNotAvailable = 15,
    /// The broker returns this error code if it receives an offset
    /// fetch or commit request for a group that it is not a
    /// coordinator for.
    NotCoordinator = 16,
    /// For a request which attempts to access an invalid topic
    /// (e.g. one which has an illegal name), or if an attempt is made
    /// to write to an internal topic (such as the consumer offsets
    /// topic).
    InvalidTopic = 17,
    /// If a message batch in a produce request exceeds the maximum
    /// configured segment size.
    RecordListTooLarge = 18,
    /// Returned from a produce request when the number of in-sync
    /// replicas is lower than the configured minimum and requiredAcks is
    /// -1.
    NotEnoughReplicas = 19,
    /// Returned from a produce request when the message was written
    /// to the log, but with fewer in-sync replicas than required.
    NotEnoughReplicasAfterAppend = 20,
    /// Returned from a produce request if the requested requiredAcks is
    /// invalid (anything other than -1, 1, or 0).
    InvalidRequiredAcks = 21,
    /// Returned from group membership requests (such as heartbeats) when
    /// the generation id provided in the request is not the current
    /// generation.
    IllegalGeneration = 22,
    /// Returned in join group when the member provides a protocol type or
    /// set of protocols which is not compatible with the current group.
    InconsistentGroupProtocol = 23,
    /// Returned in join group when the groupId is empty or null.
    InvalidGroupId = 24,
    /// Returned from group requests (offset commits/fetches, heartbeats,
    /// etc) when the memberId is not in the current generation.
    UnknownMemberId = 25,
    /// Return in join group when the requested session timeout is outside
    /// of the allowed range on the broker
    InvalidSessionTimeout = 26,
    /// Returned in heartbeat requests when the coordinator has begun
    /// rebalancing the group. This indicates to the client that it
    /// should rejoin the group.
    RebalanceInProgress = 27,
    /// This error indicates that an offset commit was rejected because of
    /// oversize metadata.
    InvalidOffsetCommitSize = 28,
    /// Returned by the broker when the client is not authorized to access
    /// the requested topic.
    TopicAuthorizationFailed = 29,
    /// Returned by the broker when the client is not authorized to access
    /// a particular groupId.
    GroupAuthorizationFailed = 30,
    /// Returned by the broker when the client is not authorized to use an
    /// inter-broker or administrative API.
    ClusterAuthorizationFailed = 31,
    /// The timestamp of the message is out of acceptable range.
    InvalidTimestamp = 32,
    /// The broker does not support the requested SASL mechanism.
    UnsupportedSaslMechanism = 33,
    /// Request is not valid given the current SASL state.
    IllegalSaslState = 34,
    /// The version of API is not supported.
    UnsupportedVersion = 35,
    /// Topic with this name already exists.
    TopicAlreadyExists = 36,
    /// Number of partitions is invalid.
    InvalidPartitions = 37,
    /// Replication-factor is invalid.
    InvalidReplicationFactor = 38,
    /// Replica assignment is invalid.
    InvalidReplicaAssignment = 39,
    /// Configuration is invalid.
    InvalidConfig = 40,
    /// This is not the correct controller for this cluster.
    NotController = 41,
    /// This most likely occurs because of a request being malformed by the client library
    /// or the message was sent to an incompatible broker. See the broker logs for more details.
    InvalidRequest = 42,
    /// The message format version on the broker does not support the request.
    UnsupportedForMessageFormat = 43,
    /// Request parameters do not satisfy the configured policy.
    PolicyViolation = 44,
    /// The broker received an out of order sequence number
    OutOfOrderSequenceNumber = 45,
    /// The broker received a duplicate sequence number
    DuplicateSequenceNumber = 46,
    /// Producer attempted an operation with an old epoch
    InvalidProducerEpoch = 47,
    /// The producer attempted a transactional operation in an invalid state
    InvalidTxnState = 48,
    /// The producer attempted to use a producer id which is not currently assigned to its
    /// transactional id
    InvalidProducerIdMapper = 49,
    /// The transaction timeout is larger than the maximum value allowed by the
    /// broker
    InvalidTransactionTimeout = 50,
    /// The producer attempted to update a transaction while another concurrent operation on the
    /// same transaction was ongoing
    ConcurrentTransactions = 51,
    /// Indicates that the transaction coordinator sending a WriteTxnMarker is no longer the
    /// current coordinator for a given producer
    TransactionCoordinatorFenced = 52,
    /// Transactional Id authorization failed
    TransactionalIdAuthorizationFailed = 53,
    /// Security features are disabled.
    SecurityDisabled = 54,
    /// The broker did not attempt to execute this operation. This may happen
    /// for batched RPCs where some operations in the batch failed, causing the
    /// broker to respond without trying the rest.
    OperationNotAttempted = 55,
    /// Disk error when trying to access log file on the disk.
    KafkaStorageError = 56,
    /// The user-specified log directory is not found in the broker config.
    LogDirNotFound = 57,
    /// SASL Authentication failed.
    SaslAuthenticationFailed = 58,
    /// This exception is raised by the broker if it could not locate the producer metadata associated with the
    /// producerId in question. This could happen if, for instance, the producer's records were deleted because their
    /// retention time had elapsed. Once the last records of the producerId are removed, the producer's metadata is
    /// removed from the broker, and future appends by the producer will return this exception.
    UnknownProducerId = 59,
    /// A partition reassignment is in progress
    ReassignmentInProgress = 60,
    /// Delegation Token feature is not enabled.
    DelegationTokenAuthDisabled = 61,
    /// Delegation Token is not found on server.
    DelegationTokenNotFound = 62,
    /// Specified Principal is not valid Owner/Renewer.
    DelegationTokenOwnerMismatch = 63,
    /// Delegation Token requests are not allowed on PLAINTEXT/1-way SSL
    /// channels and on delegation token authenticated channels.
    DelegationTokenRequestNotAllowed = 64,
    /// Delegation Token authorization failed.
    DelegationTokenAuthorizationFailed = 65,
    /// Delegation Token is expired.
    DelegationTokenExpired = 66,
    /// Supplied principalType is not supported
    InvalidPrincipalType = 67,
    /// The group The group is not empty is not empty
    NonEmptyGroup = 68,
    /// The group id The group id does not exist was not found
    GroupIdNotFound = 69,
    /// The fetch session ID was not found
    FetchSessionIdNotFound = 70,
    /// The fetch session epoch is invalid
    InvalidFetchSessionEpoch = 71,
    /// There is no listener on the leader broker that matches the listener on which metadata request was processed
    ListenerNotFound = 72,
    /// Topic deletion is disabled
    TopicDeletionDisabled = 73,
    /// The leader epoch in the request is older than the epoch on the broker
    FencedLeaderEpoch = 74,
    /// The leader epoch in the request is newer than the epoch on the broker
    UnknownLeaderEpoch = 75,
    /// The requesting client does not support the compression type of given partition
    UnsupportedCompressionType = 76,
    /// Broker epoch has changed
    StaleBrokerEpoch = 77,
    /// The leader high watermark has not caught up from a recent leader election so the offsets cannot be guaranteed to be monotonically increasing
    OffsetNotAvailable = 78,
    /// The group member needs to have a valid member id before actually entering a consumer group
    MemberIdRequired = 79,
    /// The preferred leader was not available
    PreferredLeaderNotAvailable = 80,
    /// Consumer group The consumer group has reached its max size. already has the configured maximum number of members
    GroupMaxSizeReached = 81,
    /// The broker rejected this static consumer since another consumer with the same group.instance.id has registered with a different member.id
    FencedInstanceId = 82,
}

impl Error {
    pub fn is_retriable(&self) -> bool {
        match *self {
            Error::CorruptMessage
            | Error::UnknownTopicOrPartition
            | Error::LeaderNotAvailable
            | Error::NotLeaderForPartition
            | Error::RequestTimedOut
            | Error::NetworkException
            | Error::CoordinatorLoadInProgress
            | Error::CoordinatorNotAvailable
            | Error::NotCoordinator
            | Error::NotEnoughReplicas
            | Error::NotEnoughReplicasAfterAppend
            | Error::NotController
            | Error::KafkaStorageError
            | Error::FetchSessionIdNotFound
            | Error::InvalidFetchSessionEpoch
            | Error::ListenerNotFound
            | Error::FencedLeaderEpoch
            | Error::UnknownLeaderEpoch
            | Error::OffsetNotAvailable
            | Error::PreferredLeaderNotAvailable => true,
            _ => false,
        }
    }

    pub fn reason(&self) -> &'static str {
        match *self {
            Error::Unknown => {
                "The server experienced an unexpected error when processing the request"
            }
            Error::None => "Ok",
            Error::OffsetOutOfRange => {
                "The requested offset is not within the range of offsets maintained by the server."
            }
            Error::CorruptMessage => {
                "This message has failed its CRC checksum, exceeds the valid size, or is otherwise corrupt."
            }
            Error::UnknownTopicOrPartition => "This server does not host this topic-partition.",
            Error::InvalidMessageSize => "The requested fetch size is invalid.",
            Error::LeaderNotAvailable => {
                "There is no leader for this topic-partition as we are in the middle of a leadership election."
            }
            Error::NotLeaderForPartition => {
                "This server is not the leader for that topic-partition."
            }
            Error::RequestTimedOut => "The request timed out.",
            Error::BrokerNotAvailable => "The broker is not available.",
            Error::ReplicaNotAvailable => {
                "The replica is not available for the requested topic-partition"
            }
            Error::MessageSizeTooLarge => {
                "The request included a message larger than the max message size the server will accept."
            }
            Error::StaleControllerEpoch => "The controller moved to another broker.",
            Error::OffsetMetadataTooLarge => {
                "The metadata field of the offset request was too large."
            }
            Error::NetworkException => {
                "The server disconnected before a response was received."
            }
            Error::CoordinatorLoadInProgress => {
                "The coordinator is loading and hence can't process requests."
            }
            Error::CoordinatorNotAvailable => "The coordinator is not available.",
            Error::NotCoordinator => "This is not the correct coordinator.",
            Error::InvalidTopic => {
                "The request attempted to perform an operation on an invalid topic."
            }
            Error::RecordListTooLarge => {
                "The request included message batch larger than the configured segment size on the server."
            }
            Error::NotEnoughReplicas => {
                "Messages are rejected since there are fewer in-sync replicas than required."
            }
            Error::NotEnoughReplicasAfterAppend => {
                "Messages are written to the log, but to fewer in-sync replicas than required."
            }
            Error::InvalidRequiredAcks => {
                "Produce request specified an invalid value for required acks."
            }
            Error::IllegalGeneration => "Specified group generation id is not valid.",
            Error::InconsistentGroupProtocol => {
                "The group member's supported protocols are incompatible with those of existing members."
            }
            Error::InvalidGroupId => "The configured groupId is invalid",
            Error::UnknownMemberId => "The coordinator is not aware of this member.",
            Error::InvalidSessionTimeout => {
                "The session timeout is not within the range allowed by the broker"
            }
            Error::RebalanceInProgress => "The group is rebalancing, so a rejoin is needed.",
            Error::InvalidOffsetCommitSize => "The committing offset data size is not valid",
            Error::TopicAuthorizationFailed => "Topic authorization failed.",
            Error::GroupAuthorizationFailed => "Group authorization failed.",
            Error::ClusterAuthorizationFailed => "Cluster authorization failed.",
            Error::InvalidTimestamp => {
                "The timestamp of the message is out of acceptable range."
            }
            Error::UnsupportedSaslMechanism => {
                "The broker does not support the requested SASL mechanism."
            }
            Error::IllegalSaslState => "Request is not valid given the current SASL state.",
            Error::UnsupportedVersion => "The version of API is not supported.",
            Error::TopicAlreadyExists => "Topic with this name already exists.",
            Error::InvalidPartitions => "Number of partitions is invalid.",
            Error::InvalidReplicationFactor => "Replication-factor is invalid.",
            Error::InvalidReplicaAssignment => "Replica assignment is invalid.",
            Error::InvalidConfig => "Configuration is invalid.",
            Error::NotController => "This is not the correct controller for this cluster.",
            Error::InvalidRequest => {
                "This most likely occurs because of a request being malformed by the client library or the message was sent to an incompatible broker."
            }
            Error::UnsupportedForMessageFormat => {
                "The message format version on the broker does not support the request."
            }
            Error::PolicyViolation => {
                "Request parameters do not satisfy the configured policy."
            }
            Error::OutOfOrderSequenceNumber => {
                "The broker received an out of order sequence number"
            }
            Error::DuplicateSequenceNumber => "The broker received a duplicate sequence number",
            Error::InvalidProducerEpoch => "Producer attempted an operation with an old epoch",
            Error::InvalidTxnState => {
                "The producer attempted a transactional operation in an invalid state"
            }
            Error::InvalidProducerIdMapper => {
                "The producer attempted to use a producer id which is not currently assigned to its transactional id"
            }
            Error::InvalidTransactionTimeout => {
                "The transaction timeout is larger than the maximum value allowed by the broker"
            }
            Error::ConcurrentTransactions => {
                "The producer attempted to update a transaction while another concurrent operation on the same transaction was ongoing"
            }
            Error::TransactionCoordinatorFenced => {
                "Indicates that the transaction coordinator sending a WriteTxnMarker is no longer the current coordinator for a given producer"
            }
            Error::TransactionalIdAuthorizationFailed => {
                "Transactional Id authorization failed"
            }
            Error::SecurityDisabled => {"Security features are disabled."}
            Error::OperationNotAttempted => {"The broker did not attempt to execute this operation."}
            Error::KafkaStorageError =>{"Disk error when trying to access log file on the disk."}
            Error::LogDirNotFound =>"The user-specified log directory is not found in the broker config.",
            Error::SaslAuthenticationFailed => "SASL Authentication failed.",
            Error::UnknownProducerId => "This exception is raised by the broker if it could not locate the producer metadata associated with the producerId in question.",
            Error::ReassignmentInProgress => "A partition reassignment is in progress",
            Error::DelegationTokenAuthDisabled => "Delegation Token feature is not enabled.",
            Error::DelegationTokenNotFound => "Delegation Token is not found on server.",
            Error::DelegationTokenOwnerMismatch => "Specified Principal is not valid Owner/Renewer.",
            Error::DelegationTokenRequestNotAllowed => "Delegation Token requests are not allowed on PLAINTEXT/1-way SSL channels and on delegation token authenticated channels.",
            Error::DelegationTokenAuthorizationFailed => "Delegation Token authorization failed.",
            Error::DelegationTokenExpired => "Delegation Token is expired.",
            Error::InvalidPrincipalType => "Supplied principalType is not supported",
            Error::NonEmptyGroup => "The group The group is not empty is not empty",
            Error::GroupIdNotFound => "The group id The group id does not exist was not found",
            Error::FetchSessionIdNotFound =>"The fetch session ID was not found",
            Error::InvalidFetchSessionEpoch => "The fetch session epoch is invalid",
            Error::ListenerNotFound => "There is no listener on the leader broker that matches the listener on which metadata request was processed",
            Error::TopicDeletionDisabled => "Topic deletion is disabled",
            Error::FencedLeaderEpoch => "The leader epoch in the request is older than the epoch on the broker",
            Error::UnknownLeaderEpoch => "The leader epoch in the request is newer than the epoch on the broker",
            Error::UnsupportedCompressionType => "The requesting client does not support the compression type of given partition",
            Error::StaleBrokerEpoch => "Broker epoch has changed",
            Error::OffsetNotAvailable => "The leader high watermark has not caught up from a recent leader election so the offsets cannot be guaranteed to be monotonically increasing",
            Error::MemberIdRequired => "The group member needs to have a valid member id before actually entering a consumer group",
            Error::PreferredLeaderNotAvailable => "The preferred leader was not available",
            Error::GroupMaxSizeReached => "Consumer group The consumer group has reached its max size. already has the configured maximum number of members",
            Error::FencedInstanceId => "The broker rejected this static consumer since another consumer with the same group.instance.id has registered with a different member.id",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "{:?}", self)
    }
}
