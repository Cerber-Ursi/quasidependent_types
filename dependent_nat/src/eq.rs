
use super::Nat;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub struct Equiv<N1: Nat, N2: Nat>(PhantomData<(N1, N2)>);

impl<N1: Nat, N2: Nat> Equiv<N1, N2> {
    pub fn rev(self) -> Equiv<N2, N1> {
        Equiv(PhantomData)
    }
    pub fn check(n1: N1, n2: N2) -> Option<Self> {
        if n1.as_usize() == n2.as_usize() {
            Some(Self(PhantomData))
        } else {
            None
        }
    }
}
impl<N: Nat> Equiv<N, N> {
    pub fn refl() -> Self {
        Self(PhantomData)
    }
}
