pub mod create;
pub mod delete;
pub mod list;
pub mod update;
pub mod view;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PocketbaseOperationError {
    #[error("Operation Failed")]
    Failed,
}
