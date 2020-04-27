use super::Nat;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct Equiv<N1: Nat, N2: Nat>(PhantomData<(N1, N2)>);

impl<N1: Nat, N2: Nat> Equiv<N1, N2> {
    pub fn rev(self) -> Equiv<N2, N1> {
        Equiv(PhantomData)
    }
    pub fn try_create() -> Option<Self> {
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
    pub fn check(_: N1, _: N2) -> Option<Self> {
        Self::try_create()
    }
}
impl<N: Nat> Equiv<N, N> {
    pub fn refl() -> Self {
        Self(PhantomData)
    }
}

pub trait NatWrapper: Nat {
    fn refl(self) -> Equiv<Self, Self> {
        Equiv::refl()
    }
}
impl<N: Nat> NatWrapper for N {}
