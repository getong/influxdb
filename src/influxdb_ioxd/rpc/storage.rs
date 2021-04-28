//! This module contains gRPC service implementatations

/// `[0x00]` is the magic value that that the storage gRPC layer uses to
/// encode a tag_key that means "measurement name"
pub(crate) const TAG_KEY_MEASUREMENT: &[u8] = &[0];

/// `[0xff]` is is the magic value that that the storage gRPC layer uses
/// to encode a tag_key that means "field name"
pub(crate) const TAG_KEY_FIELD: &[u8] = &[255];

pub mod data;
pub mod expr;
pub mod id;
pub mod input;
pub mod service;

use generated_types::storage_server::{Storage, StorageServer};
use query::DatabaseStore;
use std::sync::Arc;

/// Concrete implementation of the gRPC InfluxDB Storage Service API
#[derive(Debug)]
struct StorageService<T: DatabaseStore> {
    pub db_store: Arc<T>,
    pub metrics_registry: Arc<metrics::MetricRegistry>,
}

pub fn make_server<T: DatabaseStore + 'static>(
    db_store: Arc<T>,
    metrics_registry: Arc<metrics::MetricRegistry>,
) -> StorageServer<impl Storage> {
    StorageServer::new(StorageService {
        db_store,
        metrics_registry,
    })
}
