pub mod operations;

pub use operations::create;
pub use operations::list;
pub use operations::view;
pub use operations::delete;

pub trait Recordable {}
pub trait Changeset {}
