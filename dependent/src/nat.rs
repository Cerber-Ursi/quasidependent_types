use dependent_attribute::randomize;

#[randomize]
mod nat {
    use std::marker::PhantomData;

    #[derive(Copy, Clone, Debug)]
    pub struct Equiv<T1: Nat, T2: Nat>(PhantomData<(T1, T2)>);

    impl<T1: Nat, T2: Nat> Equiv<T1, T2> {
        pub fn rev(self) -> Equiv<T2, T1> {
            Equiv(PhantomData)
        }
    }

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
    macro_rules! n {
        () => {
            #[derive(Copy, Clone)]
            struct N(usize);
            impl $crate::traits::NatInner for N {}
            impl $crate::traits::Nat for N {
                fn as_usize(&self) -> usize {
                    self.0
                }
                fn from_usize(s: usize) -> Self {
                    Self(s)
                }
            }
        };
    }

}
