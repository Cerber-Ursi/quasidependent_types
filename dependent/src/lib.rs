pub mod traits {
    pub use super::dependent::*;
    pub use super::nat::*;
}

mod dependent {    
    pub trait DependentInner: Sized + Clone {
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
        fn modify(&mut self, f: impl FnOnce(&mut Self::Frozen)) {
            f(self.freeze());
        }
    }
    impl<T: DependentInner> DependentInnerOperate for T {}

    pub trait Dependent {
        type Inner: DependentInnerOperate;
        fn from_inner(_: Self::Inner) -> Self
        where
            Self: Sized;
        fn into_inner(self) -> Self::Inner;
        fn inner(&self) -> &Self::Inner;
        fn map(self, f: impl FnOnce(&mut <Self::Inner as DependentInner>::Frozen)) -> Self
        where
            Self: Sized,
        {
            let mut inner = self.into_inner();
            f(inner.freeze());
            Self::from_inner(inner)
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
            Self::from_inner(inner)
        }
        fn consume_ref(&self, f: impl FnOnce(&mut <Self::Inner as DependentInner>::Frozen)) {
            f(self.inner().clone().freeze())
        }
    }
}

mod nat {
    use std::marker::PhantomData;

    #[derive(Copy, Clone, Debug)]
    pub struct Equiv<T1: Nat, T2: Nat>(PhantomData<(T1, T2)>);

    impl<T1: Nat, T2: Nat> Equiv<T1, T2> {
        pub fn rev(self) -> Equiv<T2, T1> {
            Equiv(PhantomData)
        }
    }

    pub trait Nat: Sized {
        fn as_usize(&self) -> usize;
        fn from_usize(s: usize) -> Self;
    }
    pub trait NatEq: Nat {
        fn eq<N: Nat>(this: Self, other: N) -> Option<Equiv<Self, N>> {
            if this.as_usize() == other.as_usize() {
                Some(Equiv(PhantomData))
            } else {
                None
            }
        }
    }
    impl<T: Nat> NatEq for T {}
}

pub mod vect {
    use super::traits::*;
    use core::iter::FromIterator;
    use std::marker::PhantomData;
    
    impl<Item: Clone> DependentInner for Vec<Item> {
        type Frozen = [Item];
        fn freeze(&mut self) -> &mut Self::Frozen {
            self.as_mut_slice()
        }
        fn recreate(frozen: &Self::Frozen) -> Self {
            frozen.iter().cloned().collect()
        }
    }

    pub struct Vect<T, N: Nat>(Vec<T>, PhantomData<N>);
    impl<Item: Clone, N: Nat> Dependent for Vect<Item, N> {
        type Inner = Vec<Item>;
        fn from_inner(v: Self::Inner) -> Self {
            Self(v, PhantomData)
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
            Self(iter.into_iter().collect(), PhantomData)
        }
    }
    
    pub fn collect<Item: Clone, N: Nat, I: IntoIterator<Item = Item>>(
        iter: I,
    ) -> (N, Vect<Item, N>) {
        let inner: Vec<_> = iter.into_iter().collect();
        (N::from_usize(inner.len()), Vect(inner, PhantomData))
    }

    impl<Item: Clone, N: Nat> Vect<Item, N> {
        pub fn retag<New: Nat>(self, _proof: Equiv<N, New>) -> Vect<Item, New> {
            collect(self.0).1
        }
    }
}