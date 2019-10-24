#![feature(proc_macro_hygiene)]

use dependent_attribute::*;
use std::ops::AddAssign;
use std::str::FromStr;

pub trait DependentInner: Sized {
    type Frozen: ?Sized;
    fn freeze(&mut self) -> &mut Self::Frozen;
    fn recreate(_: &Self::Frozen) -> Self;
}
pub trait DependentInnerOperate: DependentInner {
    fn transform(mut self, f: impl FnOnce(&mut Self::Frozen)) -> Self {
        let mut tmp = Self::recreate(self.freeze());
        f(tmp.freeze());
        tmp
    }
}
impl<T: DependentInner> DependentInnerOperate for T {}
impl<Item: Clone> DependentInner for Vec<Item> {
    type Frozen = [Item];
    fn freeze(&mut self) -> &mut Self::Frozen {
        self.as_mut_slice()
    }
    fn recreate(frozen: &Self::Frozen) -> Self {
        frozen.iter().cloned().collect()
    }
}

pub trait DependentVec<Inner: DependentInnerOperate>: Sized {
    fn try_unify<T: DependentVec<Inner>>(self, other: T) -> Result<(Self, Self), (Self, T)>;
    fn len(&self) -> usize;
    fn into_inner(self) -> Inner;
    fn map(self, f: impl FnOnce(&mut Inner::Frozen)) -> Self;
    fn consume(self, f: impl FnOnce(&mut Inner::Frozen));
}

pub struct Vect<Item>(Vec<Item>);
impl<Item: Clone> DependentVec<Vec<Item>> for Vect<Item> {
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
    fn map(self, f: impl FnOnce(&mut [Item])) -> Self {
        let mut inner = self.into_inner();
        f(inner.freeze());
        Self(inner)
    }
    fn consume(self, f: impl FnOnce(&mut [Item])) {
       f(self.into_inner().freeze()) 
    }
}

impl<Item> From<Vec<Item>> for Vect<Item> {
    fn from(input: Vec<Item>) -> Self {
        Self(input)
    }
}

#[dependent_out(DependentVec<Vec<Item>>, Vect<Item>)]
pub fn parse_list_discarding<Item: FromStr + Clone>(input: String) -> Vec<Item> {
    input
        .split(",")
        .map(str::parse)
        .filter_map(Result::ok)
        .collect()
}

pub fn zip_add<Item: AddAssign<Item> + ToOwned, T: DependentVec<Vec<Item>>>(first: T, second: T) -> T {
    first.map(|v1| second.consume(|v2| v1.iter_mut().zip(v2.iter()).for_each(|(i1, i2)| *i1 += i2.to_owned())))
}

#[test]
fn test() {
    let v1 = parse_list_discarding!([i64] "1,2,3,str,4".into());
    let v2 = parse_list_discarding!([i64] "-1,-2,str,-3,-4".into());
    let (v1, v2) = v1.try_unify(v2).unwrap_or_else(|_| panic!("Not equal length"));
    assert_eq!(zip_add(v1, v2).into_inner(), vec![0, 0, 0, 0]);
}
