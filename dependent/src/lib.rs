//! Example crate for the simulation of dependent typing.

pub mod traits {
    pub use super::dependent::*;
    pub use super::nat::*;
}

mod dependent;

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
