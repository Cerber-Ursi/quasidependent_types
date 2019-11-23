use dependent::traits::*;
use core::iter::FromIterator;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct VecWrapper<Item, N: Nat>(Vec<Item>, PhantomData<N>);
impl<Item: Clone, N: Nat> VecWrapper<Item, N> {
    pub fn into_inner(self) -> Vec<Item> {
        self.0
    }
}
impl<Item: Clone, N: Nat> Clone for VecWrapper<Item, N> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}
impl<Item: Clone, N: Nat> FromIterator<Item> for VecWrapper<Item, N> {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        Self(iter.into_iter().collect(), PhantomData)
    }
}
impl<Item: Clone, N: Nat> IntoIterator for VecWrapper<Item, N> {
    type Item = Item;
    type IntoIter = std::vec::IntoIter<Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<Item: Clone, N: Nat> DependentInner for VecWrapper<Item, N> {
    type Frozen = [Item];
    fn freeze(&mut self) -> &mut Self::Frozen {
        self.0.as_mut_slice()
    }
    fn recreate(frozen: &Self::Frozen, _: Marker) -> Self {
        frozen.iter().cloned().collect()
    }
}

pub struct Vect<T, N: Nat>(VecWrapper<T, N>);
impl<Item: Clone, N: Nat> Dependent for Vect<Item, N> {
    type Inner = VecWrapper<Item, N>;
    fn from_inner(v: Self::Inner, _: Marker) -> Self {
        Self(v)
    }
    fn into_inner(self) -> Self::Inner {
        self.0
    }
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
}
impl<Item: Clone, N: Nat> FromIterator<Item> for Vect<Item, N> {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}
pub fn collect<Item: Clone, N: Nat, I: IntoIterator<Item = Item>>(iter: I) -> (N, Vect<Item, N>) {
    let inner: VecWrapper<_, _> = iter.into_iter().collect();
    (N::from_usize(inner.0.len()), Vect(inner))
}

impl<Item: Clone, N: Nat> Vect<Item, N> {
    pub fn retag<New: Nat>(self, _proof: Equiv<N, New>) -> Vect<Item, New> {
        collect(self.0).1
    }
}
