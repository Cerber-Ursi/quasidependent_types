#[macro_use]
pub mod nat {

    /// Inner trait, not to be used by consumers directly. Its name is labeled with timestamp on every build.
    pub trait NatInner {}
    pub trait Nat: Sized + NatInner + Clone + Copy {
        fn get_usize() -> Option<usize>;
        fn as_usize(self) -> usize;
        fn from_usize(s: usize) -> Result<Self, ()>;
        fn get() -> Self;
        fn try_get() -> Option<Self>;
    }

    #[macro_export]
    macro_rules! with_n {
        ($($inner:tt)*) => {{
            #[derive(Copy, Clone)]
            struct N;
            impl $crate::NatInner for N {}
            impl $crate::Nat for N {
                fn get_usize() -> Option<usize> {
                    Some(0)
                }
                fn as_usize(self) -> usize {
                    0 
                }
                fn from_usize(s: usize) -> Result<Self, ()> {
                    Ok(Self)
                }
                fn get() -> Self {
                    Self
                }
                fn try_get() -> Option<Self> {
                    Some(Self)
                }
            }
            $($inner)*
        }};
    }

}
