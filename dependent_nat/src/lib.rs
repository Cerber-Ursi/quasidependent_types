pub fn expect_nat<N: Nat>(s: usize) -> N {
    N::from_usize(s).unwrap()
}

pub trait Nat: Sized + Clone + Copy {
    fn from_usize(s: usize) -> Result<Self, ()>;
}

#[macro_export]
macro_rules! with_n {
        ($($inner:tt)*) => {{
            #[derive(Copy, Clone)]
            struct N;
            impl $crate::Nat for N {
                fn from_usize(s: usize) -> Result<Self, ()> {
                    Ok(Self)
                }
            }
            $($inner)*
        }};
    }
