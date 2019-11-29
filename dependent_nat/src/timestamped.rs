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
            #[derive(Copy, Clone)]
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
                    Self::try_get().expect("Trying to create `Add` instance which is yet undefined")
                }
                fn try_get() -> Option<Self> {
                    Self::get_usize().map(|_| Self)
                }
            }
            $($inner)*
        }};
    }

}
