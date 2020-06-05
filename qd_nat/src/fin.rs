use crate::Nat;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
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
        if self.0 < N::get_usize().unwrap() {
            let ret = Some(Fin(self.0, PhantomData));
            self.0 += 1;
            ret
        } else {
            None
        }
    }


    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0.checked_sub(N::get_usize().unwrap()) {
            Some(hint) => (hint, Some(hint)),
            None => (usize::MAX, None),
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Fin<N>> {
        if let Some(plus_n) = self.0.checked_add(n) {
            if plus_n < N::get_usize().unwrap() {
                self.0 = plus_n + 1;
                return Some(Fin(plus_n, PhantomData));
            }
        }
        self.0 = N::get_usize().unwrap();
        None
    }
}

pub trait NatIterUntil: Nat {
    fn iter_until() -> IterUntil<Self> {
        IterUntil(0, PhantomData)
    }
    fn iter_until_this(self) -> IterUntil<Self> {
        Self::iter_until()
    }
}
impl<N: Nat> NatIterUntil for N {}
