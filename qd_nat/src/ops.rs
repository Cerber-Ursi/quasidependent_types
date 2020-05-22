use crate::{Equiv, Nat, NatStoreError};
use qd_core::{Deducible, StaticallyProvable};
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct Add<N1: Nat, N2: Nat>(PhantomData<(N1, N2)>);
impl<N1: Nat, N2: Nat> Nat for Add<N1, N2> {
    fn get_usize() -> Option<usize> {
        N1::get_usize().and_then(|n1| N2::get_usize().map(|n2| n1 + n2))
    }
    fn as_usize(self) -> usize {
        Self::get_usize().expect("`Add` was created and queried before its components were set")
    }
    fn from_usize(s: usize) -> Result<Self, NatStoreError> {
        if Self::get_usize() == Some(s) {
            Ok(Self(PhantomData))
        } else {
            unimplemented!(
                "I'm not sure how to handle this error, but this should not be called anyway"
            );
        }
    }
    fn get() -> Self {
        Self::try_get().expect("Trying to create `Add` instance which is yet undefined")
    }
    fn try_get() -> Option<Self> {
        Self::get_usize().map(|_| Self(PhantomData))
    }
}

// here can be more implementations (Sub, Mul etc.), but let us stop here for now

impl<N1: Nat, N2: Nat> StaticallyProvable for Equiv<Add<N1, N2>, Add<N2, N1>> {
    fn proof() -> Self {
        Self(PhantomData)
    }
}

impl<M: Nat, N1: Nat, N2: Nat> Deducible<Equiv<N1, N2>> for Equiv<Add<M, N1>, Add<M, N2>> {
    fn deduce(_: Equiv<N1, N2>) -> Self {
        Self(PhantomData)
    }
}
