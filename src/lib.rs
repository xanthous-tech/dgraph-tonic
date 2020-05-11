pub use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint, Identity};
pub use tonic::Status;

#[cfg(feature = "dgraph-1-0")]
pub use crate::api::Assigned;
use crate::api::IDgraphClient;
pub use crate::api::{
    Check, LoginRequest, Mutation, Operation, Payload, Request, Response, TxnContext, Version,
};
pub use crate::client::Endpoints;
#[cfg(feature = "acl")]
pub use crate::client::{AclClient, AclClientType, LazyDefaultChannel, TxnAcl};
#[cfg(all(feature = "acl", feature = "tls"))]
pub use crate::client::{AclTlsClient, TxnAclTls};
pub use crate::client::{Client, Txn};
#[cfg(feature = "tls")]
pub use crate::client::{TlsClient, TxnTls};
pub use crate::errors::{ClientError, DgraphError};
pub use crate::txn::{
    BestEffortTxn, Mutate, MutatedTxn, MutationResponse, Query, ReadOnlyTxn, TxnState, TxnType,
    TxnVariant,
};

mod api;
mod client;
mod errors;
mod stub;
#[cfg(feature = "sync")]
pub mod sync;
mod txn;

pub type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = StdError> = ::std::result::Result<T, E>;
