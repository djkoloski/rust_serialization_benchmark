pub mod log;
pub mod mesh;
pub mod minecraft_savedata;
pub mod mk48;

/// Trait for test data types that have a form with borrowed fields.
pub trait BorrowableData: Sized + PartialEq {
    type Borrowed<'a>: PartialEq + From<&'a Self> + Into<Self>
    where
        Self: 'a;
}
