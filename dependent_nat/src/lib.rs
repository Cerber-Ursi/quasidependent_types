#![feature(proc_macro_hygiene)]
#![feature(log_syntax)]

mod eq;
mod holder;

#[cfg(feature = "nat_ops")]
mod ops;

mod timestamped;

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
pub use self::timestamped::nat::*;

#[cfg(feature = "nat_ops")]
pub use self::ops::*;

