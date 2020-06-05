use super::{Nat, Primitive};
use qd_core::{Deducible, StaticallyProvable};
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct Equiv<N1: Nat, N2: Nat>(pub(crate) PhantomData<(N1, N2)>);

impl<N1: Nat, N2: Nat> Equiv<N1, N2> {
    pub fn rev(self) -> Equiv<N2, N1> {
        Equiv::deduce(self)
    }
    pub fn check() -> Option<Self> {
        N1::get_usize().and_then(|n1| {
            N2::get_usize().and_then(|n2| {
                if n1 == n2 {
                    Some(Self(PhantomData))
                } else {
                    None
                }
            })
        })
    }
    pub fn try_prove_for(_: N1, _: N2) -> Option<Self> {
        Self::check()
    }
}

impl<N: Nat> Equiv<N, N> {
    pub fn refl() -> Self {
        Self(PhantomData)
    }
}
pub trait NatWrapper: Nat {
    fn refl(self) -> Equiv<Self, Self> {
        Equiv(PhantomData)
    }
}
impl<N: Nat> NatWrapper for N {}

impl<N: Primitive> StaticallyProvable for Equiv<N, N> {
    fn proof() -> Self {
        Self::refl()
    }
}

type Transit<N1, N2, N3> = (Equiv<N1, N2>, Equiv<N2, N3>);
impl<N1: Nat, N2: Nat, N3: Nat> Deducible<Transit<N1, N2, N3>> for Equiv<N1, N3> {
    fn deduce(_: Transit<N1, N2, N3>) -> Self {
        Self(PhantomData)
    }
}

impl<N1: Nat, N2: Nat> Deducible<Equiv<N1, N2>> for Equiv<N2, N1> {
    fn deduce(_: Equiv<N1, N2>) -> Self {
        Self(PhantomData)
    }
}
