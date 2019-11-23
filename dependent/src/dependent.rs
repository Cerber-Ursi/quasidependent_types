struct InnerMarker;
pub struct Marker(InnerMarker);
macro_rules! m {
    () => {Marker(InnerMarker)}
}

/// Main trait, describing possible dependent type.
pub trait Dependent {
    type Native;
    type Frozen: ?Sized;
    fn freeze(&self) -> &Self::Frozen;
    fn freeze_mut(&mut self) -> &mut Self::Frozen;
    fn from_frozen(_: &Self::Frozen, _: Marker) -> Self
    where
        Self: Sized;
    fn from_native(_: Self::Native, _: Marker) -> Self
    where
        Self: Sized;
    fn into_native(self) -> Self::Native;
    fn as_native(&self) -> &Self::Native;
}
pub trait DependentOperate: Dependent {
    fn map_clone(&self, f: impl FnOnce(&mut Self::Frozen)) -> Self
    where
        Self: Sized,
        Self::Native: Clone
    {
        let mut cloned = Self::from_native(self.as_native().clone(), m!());
        f(cloned.freeze_mut());
        cloned
    }
}
impl<T: Dependent> DependentOperate for T {}
