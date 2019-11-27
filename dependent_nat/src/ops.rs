
use crate::{Nat, NatStoreError};
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
            unimplemented!("I'm not sure how to handle this error, but this should not be callable anyway");
        }
    }
}
