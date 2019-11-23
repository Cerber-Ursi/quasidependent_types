//! Example crate for the simulation of dependent typing.

#![feature(proc_macro_hygiene)]

pub mod traits {
    pub use super::dependent::*;
    pub use super::nat::*;
}

mod dependent;
mod nat;