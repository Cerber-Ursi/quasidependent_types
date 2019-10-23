#![feature(proc_macro_hygiene)]

use typenum::Unsigned;
use std::marker::PhantomData;
use dependent_attribute::*;

pub struct Vect<Item, Size>(Vec<Item>, PhantomData<Size>);

pub trait DependentVec<Item> {
    fn try_as_fixed<Size: Unsigned>(self) -> Option<Vect<Item, Size>>;
}

#[dependent_out]
pub fn collect_from<Item>(iter: impl Iterator<Item = Item>) -> Vec<Item> {
    iter.collect()
}
