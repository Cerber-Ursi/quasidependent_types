#![feature(proc_macro_hygiene)]

use dependent_attribute::*;
use std::ops::Add;
use std::str::FromStr;

pub trait DependentVec<Inner>: Sized {
    fn try_unify<T: DependentVec<Inner>>(self, other: T) -> Result<(Self, Self), (Self, T)>;
    fn len(&self) -> usize;
    fn into_inner(self) -> Inner;
    fn map2(self, other: Self, f: impl FnOnce(Inner, Inner) -> Inner) -> Self;
}

pub struct Vect<Item>(Vec<Item>);
impl<Item> DependentVec<Vec<Item>> for Vect<Item> {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn into_inner(self) -> Vec<Item> {
        self.0
    }
    fn try_unify<T: DependentVec<Vec<Item>>>(self, other: T) -> Result<(Self, Self), (Self, T)> {
        if self.len() == other.len() {
            let other = Self(other.into_inner());
            Ok((self, other))
        } else {
            Err((self, other))
        }
    }
    fn map2(self, other: Self, f: impl FnOnce(Vec<Item>, Vec<Item>) -> Vec<Item>) -> Self {
        Self(f(self.into_inner(), other.into_inner()))
    }
}

impl<Item> From<Vec<Item>> for Vect<Item> {
    fn from(input: Vec<Item>) -> Self {
        Self(input)
    }
}

#[dependent_out(DependentVec<Vec<Item>>, Vect<Item>)]
pub fn parse_list_discarding<Item: FromStr>(input: String) -> Vec<Item> {
    input
        .split(",")
        .map(str::parse)
        .filter_map(Result::ok)
        .collect()
}

pub fn zip_add<Item: Add<Item, Output = Item>, T: DependentVec<Vec<Item>>>(first: T, second: T) -> T {
    first.map2(second, |v1, v2| v1.into_iter().zip(v2.into_iter()).map(|(i1, i2)| i1 + i2).collect())
}

#[test]
fn test() {
    let v1 = parse_list_discarding!([i64] "1,2,3,str,4".into());
    let v2 = parse_list_discarding!([i64] "-1,-2,str,-3,-4".into());
    let (v1, v2) = v1.try_unify(v2).unwrap_or_else(|_| panic!("Not equal length"));
    assert_eq!(zip_add(v1, v2).into_inner(), vec![0, 0, 0, 0]);}
