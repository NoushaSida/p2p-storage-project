#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUser {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub surname: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub salt: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub registration_date: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUsername {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPeerId {
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDevice {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub device_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub disk_size: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub mount_point: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub country: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub registration_date: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFileId {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub file_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFileInfo {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub file_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub file_size: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub file_size_compressed: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub file_type: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub file_status: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub upload_date: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "9")]
    pub file_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "10")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub write: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChunk {
    #[prost(string, tag = "1")]
    pub file_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub piece_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub chunk_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub chunk_bytes: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMetrics {
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub uptime_start: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub uptime_end: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub disk_read: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub disk_write: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub throughput: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiveness {
    #[prost(string, tag = "1")]
    pub peer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPermissions {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub file_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub write: ::prost::alloc::string::String,
}
