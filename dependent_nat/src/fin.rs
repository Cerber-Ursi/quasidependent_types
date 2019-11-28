use crate::Nat;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
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
}

pub struct IterUntil<N: Nat>(usize, PhantomData<N>);
impl<N: Nat> Iterator for IterUntil<N> {
    type Item = Fin<N>;
    fn next(&mut self) -> Option<Fin<N>> {
        let ret = if self.0 < N::get_usize().unwrap() {
            Some(Fin(self.0, PhantomData))
        } else {
            None
        };
        self.0 += 1;
        ret
    }
}

pub trait NatIterUntil: Nat {
    fn iter_until() -> IterUntil<Self> {
        IterUntil(0, PhantomData)
    }
}
impl<N: Nat> NatIterUntil for N {}
