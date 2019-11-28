use crate::Nat;
use std::marker::PhantomData;

pub struct Fin<N: Nat>(usize, PhantomData<N>);

impl<N: Nat> Fin<N> {
    pub fn from_usize(s: usize) -> Option<Self> {
        if s < N::get_usize()? {
            Some(Self(s, PhantomData))
        } else {
            None
        }
    }
    pub fn as_usize(self) -> usize {
        self.0
    }
    #[cfg(feature = "nat_ops")]
    pub fn from_proof<N2: Nat>(_proof: crate::Less<N2, N>) -> Option<Self> {
        N2::get_usize().and_then(Self::from_usize)
    }
}
