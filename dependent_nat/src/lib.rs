#![feature(proc_macro_hygiene)]

mod timestamped;

pub fn expect_nat<N: Nat>(s: usize) -> N {
    N::from_usize(s).unwrap()
}

pub use self::timestamped::nat::*;

