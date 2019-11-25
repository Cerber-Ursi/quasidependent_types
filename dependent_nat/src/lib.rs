#![feature(proc_macro_hygiene)]

use dependent_attribute::label_timestamp;

#[label_timestamp(NatInner)]
mod nat {
    use std::marker::PhantomData;

    #[derive(Copy, Clone, Debug)]
    pub struct Equiv<T1: Nat, T2: Nat>(PhantomData<(T1, T2)>);

    impl<T1: Nat, T2: Nat> Equiv<T1, T2> {
        pub fn rev(self) -> Equiv<T2, T1> {
            Equiv(PhantomData)
        }
    }

    /// Inner trait, not to be used by consumers directly. Its name is labeled with timestamp on every build.
    pub trait NatInner {}
    pub trait Nat: Sized + NatInner {
        fn as_usize(&self) -> usize;
        fn from_usize(s: usize) -> Self;
    }
    pub trait NatEq: Nat {
        fn eq<N: Nat>(this: Self, other: N) -> Option<Equiv<Self, N>> {
            if this.as_usize() == other.as_usize() {
                Some(Equiv(PhantomData))
            } else {
                None
            }
        }
    }
    impl<T: Nat> NatEq for T {}

    #[macro_export]
    macro_rules! with_n {
        ($($inner:tt)*) => {{
            #[derive(Copy, Clone)]
            struct N(usize);
            impl $crate::NatInner for N {}
            impl $crate::Nat for N {
                fn as_usize(&self) -> usize {
                    self.0
                }
                fn from_usize(s: usize) -> Self {
                    Self(s)
                }
            }
            $($inner)*
        }};
    }

    #[cfg(feature = "typenum_consts")]
    mod typenum_consts {
        use typenum::Unsigned;
        use super::*;
        impl<T: Unsigned> NatInner for T {}
        impl<T: Unsigned + Default> Nat for T {
            fn as_usize(&self) -> usize {
                Self::USIZE
            }
            fn from_usize(s: usize) -> Self {
                if s == Self::USIZE {
                    Self::default()
                } else {
                    panic!(format!("Runtime value mismatched with compile-time constraint: expected {}, got {}", Self::USIZE, s));
                }
            }
        }
    }
    #[cfg(feature = "typenum_consts")]
    pub use self::typenum_consts::*;

}

pub use self::nat::*;
