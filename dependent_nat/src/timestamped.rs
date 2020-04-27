use dependent_attribute::label_timestamp;

#[label_timestamp(NatInner)]
#[macro_use]
pub mod nat {
    /// Inner trait, not to be used by consumers directly. Its name is labeled with timestamp on every build.
    pub trait NatInner {}
    pub trait Nat: Sized + NatInner + Clone + Copy {
        fn get_usize() -> Option<usize>;
        fn as_usize(self) -> usize;
        fn from_usize(s: usize) -> Result<Self, crate::NatStoreError>;
        fn get() -> Self;
        fn try_get() -> Option<Self>;
    }

    #[macro_export]
    macro_rules! with_n {
        ($($inner:tt)*) => {{
            use $crate::NatHolder;
            static HOLDER: NatHolder = NatHolder::new();
            #[derive(Copy, Clone, Debug)]
            struct N;
            impl $crate::NatInner for N {}
            impl $crate::Nat for N {
                fn get_usize() -> Option<usize> {
                    HOLDER.read()
                }
                fn as_usize(self) -> usize {
                    HOLDER.read().expect(concat!("Nat value was created without setting its value. Please report this bug to ", env!("CARGO_PKG_REPOSITORY"), "/issues"))
                }
                fn from_usize(s: usize) -> Result<Self, $crate::NatStoreError> {
                    HOLDER.store(s).map(|_| Self)
                }
                fn get() -> Self {
                    Self::try_get().expect("Trying to `get` the number which is yet undefined")
                }
                fn try_get() -> Option<Self> {
                    Self::get_usize().map(|_| Self)
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
            fn as_usize(self) -> usize {
                Self::USIZE
            }
            fn from_usize(s: usize) -> Result<Self, NatStoreError> {
                if s == Self::USIZE {
                    Ok(Self::default())
                } else {
                    Err(NatStoreError::AlreadyStored(Self::USIZE))
                }
            }
            fn get() -> Self {
                Self::default()
            }
            fn try_get() -> Option<Self> {
                Some(Self::default())
            }
        }
    }
    #[cfg(feature = "typenum_consts")]
    pub use self::typenum_consts::*;

    #[cfg(feature = "nat_ops")]
    mod nat_ops {
        use super::*;
        use crate::ops::*;
        impl<N1: Nat, N2: Nat> NatInner for Add<N1, N2> {}
    }
}

#[cfg(test)]
mod test {

    use super::nat::*;

    #[test]
    fn smoke() {
        let _ = with_n! {
            N::from_usize(1)
        };
    }
}
