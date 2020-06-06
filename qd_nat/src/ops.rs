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
        if let Some(inner) = Self::get_usize() {
            if inner == s {
                Ok(Self(PhantomData))
            } else {
                Err(NatStoreError::AlreadyStored(inner, s))
            }
        } else {
            Err(NatStoreError::UnknownCompositeParts(std::any::type_name::<
                Self,
            >()))
        }
    }
    fn get() -> Self {
        Self::try_get().expect("Trying to create `Add` instance which is yet undefined")
    }
    fn try_get() -> Option<Self> {
        Self::get_usize().map(|_| Self(PhantomData))
    }
}
impl<N1: Nat, N2: Nat> Add<N1, N2> {
    pub fn sum(_: N1, _: N2) -> Self {
        Self::get()
    }
}

// here can be more implementations (Sub, Mul etc.), but let us stop here for now

pub struct AddSymmetric;
impl<N1: Nat, N2: Nat> StaticallyProvable<AddSymmetric> for Equiv<Add<N1, N2>, Add<N2, N1>> {
    fn proof() -> Self {
        Self(PhantomData)
    }
}

pub struct Associative;
impl<N1: Nat, N2: Nat, N3: Nat> StaticallyProvable<Associative>
    for Equiv<Add<N1, Add<N2, N3>>, Add<Add<N1, N2>, N3>>
{
    fn proof() -> Self {
        Self(PhantomData)
    }
}

impl<M: Nat, N1: Nat, N2: Nat> Deducible<Equiv<N1, N2>> for Equiv<Add<M, N1>, Add<M, N2>> {
    fn deduce(_: Equiv<N1, N2>) -> Self {
        Self(PhantomData)
    }
}

#[cfg(feature = "typenum_consts")]
mod impl_typenum {
    use crate::{Add as NatAdd, Equiv, Nat};
    use qd_core::StaticallyProvable;
    use std::marker::PhantomData;
    use std::ops::Add;
    use typenum::Unsigned;

    pub struct Const;
    impl<N1: Unsigned + Nat, N2: Unsigned + Nat, N3: Unsigned + Nat> StaticallyProvable<Const>
        for Equiv<NatAdd<N1, N2>, N3>
    where
        N1: Add<N2, Output = N3>,
    {
        fn proof() -> Self {
            Self(PhantomData)
        }
    }
}
