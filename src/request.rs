// RequestMessage => ApiKey ApiVersion CorrelationId ClientId RequestMessage
//   ApiKey => int16
//   ApiVersion => int16
//   CorrelationId => int32
//   ClientId => string
//   RequestMessage => MetadataRequest | ProduceRequest | FetchRequest | OffsetRequest | OffsetCommitRequest | OffsetFetchRequest

pub struct Request {
    /// The `size` field gives the size of the subsequent request message in
    /// bytes.
    ///
    /// The client can read requests by first reading this 4 byte size as an
    /// integer N, and then reading and parsing the subsequent N bytes of the
    /// request.
    size: i32,
    header: RequestHeader,
    message: RequestMessage,
}

// Request Header => api_key api_version correlation_id client_id
//   api_key => INT16
//   api_version => INT16
//   correlation_id => INT32
//   client_id => NULLABLE_STRING
pub struct RequestHeader {
    /// The id of the request type
    api_key: ApiKey,
    /// The version of the API
    api_version: i16,
    /// A user-supplied integer value that will be passed back with the response.
    correlation_id: i32,
    /// A user specified identifier for the client making the request.
    client_id: Option<String>,
}
