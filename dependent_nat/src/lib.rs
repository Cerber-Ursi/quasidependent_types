mod eq;
mod holder;

#[cfg(feature = "nat_ops")]
mod ops;

#[cfg(feature = "fin")]
mod fin;

mod timestamped;

pub fn expect_nat<N: Nat>(s: usize) -> N {
    N::from_usize(s).unwrap_or_else(|err| panic!(format!("{}", err)))
}

pub use self::eq::*;
pub use self::holder::*;
pub use self::timestamped::nat::*;

#[cfg(feature = "nat_ops")]
pub use self::ops::*;

#[cfg(feature = "fin")]
pub use self::fin::*;
