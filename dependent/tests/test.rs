#![feature(proc_macro_hygiene)]

use dependent::{DependentVec, Vect};
use dependent_attribute::dependent_out;
use std::ops::AddAssign;
use std::str::FromStr;

#[dependent_out(DependentVec<Vec<Item>>, Vect<Item>)]
pub fn parse_list_discarding<Item: FromStr + Clone>(input: String) -> Vec<Item> {
    input
        .split(",")
        .map(str::parse)
        .filter_map(Result::ok)
        .collect()
}

pub fn zip_add<Item: AddAssign<Item> + Clone, T: DependentVec<Vec<Item>>>(first: &T, second: &T) -> T {
    first.map_ref(|v1| second.consume_ref(|v2| v1.iter_mut().zip(v2.iter()).for_each(|(i1, i2)| *i1 += i2.clone())))
}

#[test]
fn test() {
    let v1 = parse_list_discarding!(<i64> "1,2,3,str,4".into());
    let v2 = parse_list_discarding!(<i64> "-1,-2,str,-3,-4".into());
    let (v1, v2) = v1.try_unify(v2).unwrap_or_else(|_| panic!("Not equal length"));
    assert_eq!(zip_add(&v1, &v2).into_inner(), vec![0, 0, 0, 0]);
}

#[test]
fn error() {
    let v1 = parse_list_discarding!(<i64> "1".into());
    let v2 = parse_list_discarding!(<i64> "".into());
    v1.try_unify(v2).err().unwrap();
}

#[test]
fn chain() {
    let v1 = parse_list_discarding!(<i64> "1,2,3,str,4".into());
    let v2 = parse_list_discarding!(<i64> "-1,-2,str,-3,-4".into());
    let (v1, v2) = v1.try_unify(v2).unwrap_or_else(|_| panic!("Not equal length"));
    let v3 = zip_add(&v1, &v2);
    assert_eq!(zip_add(&v1, &v3).inner(), v1.inner());
}
