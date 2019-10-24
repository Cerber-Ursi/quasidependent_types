use dependent_attribute::dependent_trait;

pub trait DependentInner: Sized {
    type Frozen: ?Sized;
    fn freeze(&mut self) -> &mut Self::Frozen;
    fn recreate(_: &Self::Frozen) -> Self;
}
pub trait DependentInnerOperate: DependentInner {
    fn transform(mut self, f: impl FnOnce(&mut Self::Frozen)) -> Self {
        let mut tmp = Self::recreate(self.freeze());
        f(tmp.freeze());
        tmp
    }
    fn operate(&mut self, f: impl FnOnce(&mut Self::Frozen)) {
        f(self.freeze());
    }
}
impl<T: DependentInner> DependentInnerOperate for T {}
impl<Item: Clone> DependentInner for Vec<Item> {
    type Frozen = [Item];
    fn freeze(&mut self) -> &mut Self::Frozen {
        self.as_mut_slice()
    }
    fn recreate(frozen: &Self::Frozen) -> Self {
        frozen.iter().cloned().collect()
    }
}

pub trait DependentVec<Inner: DependentInnerOperate>: Sized {
    fn try_unify<T: DependentVec<Inner>>(self, other: T) -> Result<(Self, Self), (Self, T)>;
    fn len(&self) -> usize;
    fn into_inner(self) -> Inner;
    fn inner(&self) -> &Inner;
    fn map(self, f: impl FnOnce(&mut Inner::Frozen)) -> Self;
    fn consume(self, f: impl FnOnce(&mut Inner::Frozen));
    fn map_ref(&self, f: impl FnOnce(&mut Inner::Frozen)) -> Self;
    fn consume_ref(&self, f: impl FnOnce(&mut Inner::Frozen));
}

pub struct Vect<Item>(Vec<Item>);
#[dependent_trait]
impl<Item: Clone> DependentVec for Vect<Item> {
    type Inner = Vec<Item>;
    fn len(&self) -> usize {
        self.0.len()
    }
    fn into_inner(self) -> Inner {
        self.0
    }
    fn inner(&self) -> &Inner {
        &self.0
    }
    fn try_unify<T: Trait>(self, other: T) -> Result<(Self, Self), (Self, T)> {
        if self.len() == other.len() {
            let other = Self(other.into_inner());
            Ok((self, other))
        } else {
            Err((self, other))
        }
    }
    fn map(self, f: impl FnOnce(&mut Inner::Frozen)) -> Self {
        let mut inner = self.into_inner();
        f(inner.freeze());
        Self(inner)
    }
    fn consume(self, f: impl FnOnce(&mut Inner::Frozen)) {
        f(self.into_inner().freeze()) 
    }
    fn map_ref(&self, f: impl FnOnce(&mut Inner::Frozen)) -> Self {
        let mut inner = self.inner().clone();
        f(inner.freeze());
        Self(inner)
    }
    fn consume_ref(&self, f: impl FnOnce(&mut Inner::Frozen)) {
        f(self.inner().clone().freeze())
    }
}

impl<Item> From<Vec<Item>> for Vect<Item> {
    fn from(input: Vec<Item>) -> Self {
        Self(input)
    }
}
