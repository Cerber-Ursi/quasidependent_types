//! Example crate for the simulation of dependent typing.

/// Main trait, describing possible dependent type.
pub trait Dependent {
    type Native;
    type Frozen: ?Sized;
    fn freeze(&self) -> &Self::Frozen;
    fn freeze_mut(&mut self) -> &mut Self::Frozen;
    fn into_native(self) -> Self::Native;
    fn as_native(&self) -> &Self::Native;
}
