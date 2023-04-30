/// Non Admin Client
pub mod client;

/// Admin Client - Mirror of Client but with admin authentication token
pub mod admin;

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

mod httpc;
