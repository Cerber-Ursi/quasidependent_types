struct InnerMarker;
pub struct Marker(InnerMarker);
fn marker() -> Marker {
    Marker(InnerMarker)
}

/// Supplementary trait, for use in mapping operations.
///
/// It should be chosen in such a way that, having an unique reference to this type,
/// it's impossible to break the contract of the original dependent type.
pub trait DependentInner: Sized + Clone {
    type Frozen: ?Sized;
    fn freeze(&mut self) -> &mut Self::Frozen;
    fn recreate(_: &Self::Frozen, _: Marker) -> Self;
}
/// Supplementary trait, providing mapping operations on dependent type
///
/// It is automatically implemented on all types satisfying [`DependentInner`] bound,
/// so that the users of the crate are unable to override the implementation.
pub trait DependentInnerOperate: DependentInner {
    fn transform(mut self, f: impl FnOnce(&mut Self::Frozen)) -> Self {
        let mut tmp = Self::recreate(self.freeze(), marker());
        f(tmp.freeze());
        tmp
    }
    fn modify(&mut self, f: impl FnOnce(&mut Self::Frozen)) {
        f(self.freeze());
    }
}
impl<T: DependentInner> DependentInnerOperate for T {}

/// Main trait, describing possible dependent type.
pub trait Dependent {
    type Inner: DependentInnerOperate;
    fn from_inner(_: Self::Inner, _: Marker) -> Self
    where
        Self: Sized;
    fn into_inner(self) -> Self::Inner;
    fn inner(&self) -> &Self::Inner;
}
pub trait DependentOperate: Dependent {
    fn map(self, f: impl FnOnce(&mut <Self::Inner as DependentInner>::Frozen)) -> Self
    where
        Self: Sized,
    {
        let mut inner = self.into_inner();
        f(inner.freeze());
        Self::from_inner(inner, marker())
    }
    fn consume(self, f: impl FnOnce(&mut <Self::Inner as DependentInner>::Frozen))
    where
        Self: Sized,
    {
        f(self.into_inner().freeze())
    }
    fn map_ref(&self, f: impl FnOnce(&mut <Self::Inner as DependentInner>::Frozen)) -> Self
    where
        Self: Sized,
    {
        let mut inner: Self::Inner = self.inner().clone();
        f(inner.freeze());
        Self::from_inner(inner, marker())
    }
    fn consume_ref(&self, f: impl FnOnce(&mut <Self::Inner as DependentInner>::Frozen)) {
        f(self.inner().clone().freeze())
    }
}
impl<T: Dependent> DependentOperate for T {}
