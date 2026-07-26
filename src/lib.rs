/// Non Admin Client
pub mod client;

/// Auth Collection Flows
pub mod auth;

/// SuperUsers Client - authenticates against the _superusers collection
pub mod superusers;

/// PocketBase Datetime Helpers
pub mod datetime;

/// Records Related Operations
pub mod records;

/// Collections Related Operations
pub mod collections;

/// Logs Related Operations
pub mod logs;

/// Settings Related Operations
pub mod settings;

/// Realtime Server [Not Available]
pub mod rts;

/// Batch API
pub mod batch;

/// Files API
pub mod files;

mod httpc;
