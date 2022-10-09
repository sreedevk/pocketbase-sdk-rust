pub mod operations;
pub mod sync_operations;

pub use operations::create;
pub use operations::list;
pub use operations::view;
pub use operations::delete;
pub use operations::update;

pub use sync_operations::sync_create;
pub use sync_operations::sync_list;
pub use sync_operations::sync_delete;
pub use sync_operations::sync_update;
pub use sync_operations::sync_view;

pub trait Changeset {}
