#![feature(proc_macro_hygiene)]

use dependent_attribute::label_timestamp;

mod eq;
mod holder;

#[cfg(feature = "nat_ops")]
mod ops;

#[label_timestamp(NatInner)]
mod nat {
    use crate::eq::Equiv;

    /// Inner trait, not to be used by consumers directly. Its name is labeled with timestamp on every build.
    pub trait NatInner {}
    pub trait Nat: Sized + NatInner + Clone + Copy {
        fn get_usize() -> Option<usize>;
        fn as_usize(&self) -> usize;
        fn from_usize(s: usize) -> Result<Self, crate::NatStoreError>;
    }
    pub trait NatWrapper: Nat {
        fn refl(self) -> Equiv<Self, Self> {
            Equiv::refl()
        }
    }
    impl<N: Nat> NatWrapper for N {}

    #[macro_export]
    macro_rules! with_n {
        ($($inner:tt)*) => {{
            use $crate::NatHolder;
            static HOLDER: NatHolder = NatHolder::new();
            #[derive(Copy, Clone)]
            struct N;
            impl $crate::NatInner for N {}
            impl $crate::Nat for N {
                fn get_usize() -> Option<usize> {
                    HOLDER.read()
                }
                fn as_usize(&self) -> usize {
                    HOLDER.read().expect(concat!("Nat value was created without setting its value. Please report this bug to ", env!("CARGO_PKG_REPOSITORY"), "/issues"))
                }
                fn from_usize(s: usize) -> Result<Self, $crate::NatStoreError> {
                    HOLDER.store(s).map(|_| Self)
                }
            }
            $($inner)*
        }};
    }

    #[cfg(feature = "typenum_consts")]
    mod typenum_consts {
        use super::*;
        use crate::NatStoreError;
        use typenum::Unsigned;
        impl<T: Unsigned> NatInner for T {}
        impl<T: Unsigned + Default + Copy + Clone> Nat for T {
            fn get_usize() -> Option<usize> {
                Some(Self::USIZE)
            }
            fn as_usize(&self) -> usize {
                Self::USIZE
            }
            fn from_usize(s: usize) -> Result<Self, NatStoreError> {
                if s == Self::USIZE {
                    Ok(Self::default())
                } else {
                    Err(NatStoreError::AlreadyStored(Self::USIZE))
                }
            }
        }
    }
    #[cfg(feature = "typenum_consts")]
    pub use self::typenum_consts::*;

    #[cfg(feature = "nat_ops")]
    impl<T: crate::ops::NatOp> NatInner for T {}
}

pub fn expect_nat<N: Nat>(s: usize) -> N {
    use NatStoreError::*;
    N::from_usize(s).unwrap_or_else(|err| match err {
        Concurrent => panic!("Attempted to concurrently create multiple instances of Nat"),
        AlreadyStored(val) => panic!(format!(
            "Attempted to override already stored value {} with {}",
            val, s
        )),
    })
}

pub use self::eq::*;
pub use self::holder::*;
pub use self::nat::*;

#[cfg(feature = "nat_ops")]
pub use self::ops::structs::*;
