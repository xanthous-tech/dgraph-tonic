#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(uint64, tag = "1")]
    pub start_ts: u64,
    #[prost(string, tag = "4")]
    pub query: std::string::String,
    /// Support for GraphQL like variables.
    #[prost(map = "string, string", tag = "5")]
    pub vars: ::std::collections::HashMap<std::string::String, std::string::String>,
    #[prost(bool, tag = "6")]
    pub read_only: bool,
    #[prost(bool, tag = "7")]
    pub best_effort: bool,
    #[prost(message, repeated, tag = "12")]
    pub mutations: ::std::vec::Vec<Mutation>,
    #[prost(bool, tag = "13")]
    pub commit_now: bool,
    #[prost(enumeration = "request::RespFormat", tag = "14")]
    pub resp_format: i32,
}
pub mod request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RespFormat {
        Json = 0,
        Rdf = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uids {
    #[prost(string, repeated, tag = "1")]
    pub uids: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOfString {
    #[prost(string, repeated, tag = "1")]
    pub value: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(bytes, tag = "1")]
    pub json: std::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub txn: ::std::option::Option<TxnContext>,
    #[prost(message, optional, tag = "3")]
    pub latency: ::std::option::Option<Latency>,
    /// Metrics contains all metrics related to the query.
    #[prost(message, optional, tag = "4")]
    pub metrics: ::std::option::Option<Metrics>,
    /// uids contains a mapping of blank_node => uid for the node. It only returns uids
    /// that were created as part of a mutation.
    #[prost(map = "string, string", tag = "12")]
    pub uids: ::std::collections::HashMap<std::string::String, std::string::String>,
    #[prost(bytes, tag = "13")]
    pub rdf: std::vec::Vec<u8>,
    #[prost(map = "string, message", tag = "14")]
    pub hdrs: ::std::collections::HashMap<std::string::String, ListOfString>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mutation {
    #[prost(bytes, tag = "1")]
    pub set_json: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub delete_json: std::vec::Vec<u8>,
    #[prost(bytes, tag = "3")]
    pub set_nquads: std::vec::Vec<u8>,
    #[prost(bytes, tag = "4")]
    pub del_nquads: std::vec::Vec<u8>,
    #[prost(message, repeated, tag = "5")]
    pub set: ::std::vec::Vec<NQuad>,
    #[prost(message, repeated, tag = "6")]
    pub del: ::std::vec::Vec<NQuad>,
    /// This is being used for upserts.
    #[prost(string, tag = "9")]
    pub cond: std::string::String,
    /// This field is a duplicate of the one in Request and placed here for convenience.
    #[prost(bool, tag = "14")]
    pub commit_now: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    #[prost(string, tag = "1")]
    pub schema: std::string::String,
    #[prost(string, tag = "2")]
    pub drop_attr: std::string::String,
    #[prost(bool, tag = "3")]
    pub drop_all: bool,
    #[prost(enumeration = "operation::DropOp", tag = "4")]
    pub drop_op: i32,
    /// If drop_op is ATTR or TYPE, drop_value holds the name of the predicate or
    /// type to delete.
    #[prost(string, tag = "5")]
    pub drop_value: std::string::String,
    /// run indexes in background.
    #[prost(bool, tag = "6")]
    pub run_in_background: bool,
}
pub mod operation {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DropOp {
        None = 0,
        All = 1,
        Data = 2,
        Attr = 3,
        Type = 4,
    }
}
/// Worker services.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payload {
    #[prost(bytes, tag = "1")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnContext {
    #[prost(uint64, tag = "1")]
    pub start_ts: u64,
    #[prost(uint64, tag = "2")]
    pub commit_ts: u64,
    #[prost(bool, tag = "3")]
    pub aborted: bool,
    /// List of keys to be used for conflict detection.
    #[prost(string, repeated, tag = "4")]
    pub keys: ::std::vec::Vec<std::string::String>,
    /// List of predicates involved in this transaction.
    #[prost(string, repeated, tag = "5")]
    pub preds: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Check {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    #[prost(string, tag = "1")]
    pub tag: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Latency {
    #[prost(uint64, tag = "1")]
    pub parsing_ns: u64,
    #[prost(uint64, tag = "2")]
    pub processing_ns: u64,
    #[prost(uint64, tag = "3")]
    pub encoding_ns: u64,
    #[prost(uint64, tag = "4")]
    pub assign_timestamp_ns: u64,
    #[prost(uint64, tag = "5")]
    pub total_ns: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metrics {
    /// num_uids is the map of number of uids processed by each attribute.
    #[prost(map = "string, uint64", tag = "1")]
    pub num_uids: ::std::collections::HashMap<std::string::String, u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NQuad {
    #[prost(string, tag = "1")]
    pub subject: std::string::String,
    #[prost(string, tag = "2")]
    pub predicate: std::string::String,
    #[prost(string, tag = "3")]
    pub object_id: std::string::String,
    #[prost(message, optional, tag = "4")]
    pub object_value: ::std::option::Option<Value>,
    #[prost(string, tag = "5")]
    pub label: std::string::String,
    #[prost(string, tag = "6")]
    pub lang: std::string::String,
    #[prost(message, repeated, tag = "7")]
    pub facets: ::std::vec::Vec<Facet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Val", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub val: ::std::option::Option<value::Val>,
}
pub mod value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Val {
        #[prost(string, tag = "1")]
        DefaultVal(std::string::String),
        #[prost(bytes, tag = "2")]
        BytesVal(std::vec::Vec<u8>),
        #[prost(int64, tag = "3")]
        IntVal(i64),
        #[prost(bool, tag = "4")]
        BoolVal(bool),
        #[prost(string, tag = "5")]
        StrVal(std::string::String),
        #[prost(double, tag = "6")]
        DoubleVal(f64),
        /// Geo data in WKB format
        #[prost(bytes, tag = "7")]
        GeoVal(std::vec::Vec<u8>),
        #[prost(bytes, tag = "8")]
        DateVal(std::vec::Vec<u8>),
        #[prost(bytes, tag = "9")]
        DatetimeVal(std::vec::Vec<u8>),
        #[prost(string, tag = "10")]
        PasswordVal(std::string::String),
        #[prost(uint64, tag = "11")]
        UidVal(u64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Facet {
    #[prost(string, tag = "1")]
    pub key: std::string::String,
    #[prost(bytes, tag = "2")]
    pub value: std::vec::Vec<u8>,
    #[prost(enumeration = "facet::ValType", tag = "3")]
    pub val_type: i32,
    /// tokens of value.
    #[prost(string, repeated, tag = "4")]
    pub tokens: ::std::vec::Vec<std::string::String>,
    /// not stored, only used for query.
    #[prost(string, tag = "5")]
    pub alias: std::string::String,
}
pub mod facet {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ValType {
        String = 0,
        Int = 1,
        Float = 2,
        Bool = 3,
        Datetime = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
    #[prost(string, tag = "1")]
    pub userid: std::string::String,
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    #[prost(string, tag = "3")]
    pub refresh_token: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Jwt {
    #[prost(string, tag = "1")]
    pub access_jwt: std::string::String,
    #[prost(string, tag = "2")]
    pub refresh_jwt: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod dgraph_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Graph response."]
    pub struct DgraphClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DgraphClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DgraphClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn login(
            &mut self,
            request: impl tonic::IntoRequest<super::LoginRequest>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Dgraph/Login");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::Request>,
        ) -> Result<tonic::Response<super::Response>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Dgraph/Query");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn alter(
            &mut self,
            request: impl tonic::IntoRequest<super::Operation>,
        ) -> Result<tonic::Response<super::Payload>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Dgraph/Alter");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn commit_or_abort(
            &mut self,
            request: impl tonic::IntoRequest<super::TxnContext>,
        ) -> Result<tonic::Response<super::TxnContext>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Dgraph/CommitOrAbort");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn check_version(
            &mut self,
            request: impl tonic::IntoRequest<super::Check>,
        ) -> Result<tonic::Response<super::Version>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Dgraph/CheckVersion");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DgraphClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DgraphClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DgraphClient {{ ... }}")
        }
    }
}
