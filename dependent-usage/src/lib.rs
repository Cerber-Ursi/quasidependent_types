use dependent::traits::*;
use std::marker::PhantomData;

pub struct Vect<T, N: Nat>(Vec<T>, PhantomData<N>);
impl<Item: Clone, N: Nat> Dependent for Vect<Item, N> {
    type Native = Vec<Item>;
    type Frozen = [Item];
    fn freeze(&self) -> &Self::Frozen {
        self.0.as_slice()
    }
    fn freeze_mut(&mut self) -> &mut Self::Frozen {
        self.0.as_mut_slice()
    }
    fn from_frozen(frozen: &Self::Frozen, _: Marker) -> Self {
        Self(frozen.iter().cloned().collect(), PhantomData)
    }
    fn from_native(v: Self::Native, _: Marker) -> Self {
        Self(v, PhantomData)
    }
    fn into_native(self) -> Self::Native {
        self.0
    }
    fn as_native(&self) -> &Self::Native {
        &self.0
    }
}

pub fn collect<Item: Clone, N: Nat, N2: Nat + From<N>, I: IntoIterator<Item = Item>>(iter: I) -> (N, Vect<Item, N>) {
    let inner: Vec<_> = iter.into_iter().collect();
    (N::from_usize(inner.len()), Vect(inner, PhantomData))
}


impl<Item: Clone, N: Nat> Vect<Item, N> {
    pub fn retag<New: Nat>(self, _proof: Equiv<N, New>) -> Vect<Item, New> {
        Vect(self.0.into_iter().collect(), PhantomData)
    }
}
