pub mod list;
pub mod create;
pub mod view;
pub mod delete;
pub mod update;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PocketbaseOperationError {
    #[error("Operation Failed")]
    Failed
}
