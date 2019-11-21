
use dependent::traits::{Dependent, Nat, NatEq};
use dependent::vect::{collect, Vect};
use std::ops::Add;

fn zip_sum<T: Clone + Add<T, Output=T>, N: Nat>(
    first: Vect<T, N>,
    second: Vect<T, N>,
) -> Vect<T, N> {
    first.map_ref(|v1| {
        second.consume_ref(|v2| {
            v1.iter_mut()
                .zip(v2.iter())
                .for_each(|(i1, i2)| *i1 = i1.clone() + i2.clone())
        })
    })
}

macro_rules! n {
    ($n:ident) => {
        #[derive(Copy, Clone)]
        struct $n(usize);
        impl Nat for $n {
            fn as_usize(&self) -> usize {
                self.0
            }
            fn from_usize(s: usize) -> Self {
                Self(s)
            }
        }
    }
}

macro_rules! vect {
    ($v:expr => $n:ident) => {{
        n!($n);
        collect::<_, $n, _>($v)
    }}
}

macro_rules! parse_i64_discarding {
    ($string:literal => $n:ident) => {{
        use std::str::FromStr;
        let string: &str = &*$string;
        n!($n);
        collect::<_, $n, _>(string.split(",").map(i64::from_str).filter_map(Result::ok))
    }}
}

#[test]
fn summing() {
    let (n1, v1) = vect!(vec![1] => N1);
    let (n2, v2) = vect!(vec![1] => N1);

    assert_eq!(
        NatEq::eq(n1, n2).map(|proof| zip_sum(v1, v2.retag(proof.rev())).into_inner()),
        Some(vec![2])
    );
}

#[test]
fn assigning() {
    let (n1, mut v1) = vect!(vec![1] => N1);
    let (n2, v2) = vect!(vec![2] => N2);

    match NatEq::eq(n1, n2) {
        Some(proof) => {
            assert_eq!(v1.into_inner(), vec![1]);
            v1 = v2.retag(proof.rev());
            assert_eq!(v1.into_inner(), vec![2]);
        }
        None => panic!("Assertion broken - mismatched sizes")
    };
}

#[test]
fn mismatch() {
    let (n1, _) = vect!(vec![1] => N1);
    let (n2, _) = vect!(vec![2, 3] => N2);
    assert!(NatEq::eq(n1, n2).is_none());
}

#[test]
fn runtime_size() {
    let (n1, v1) = parse_i64_discarding!("1,2,3,four,5" => N1);
    let (n2, v2) = parse_i64_discarding!("1,two,3,4,5" => N2);
    assert_eq!(
        NatEq::eq(n1, n2).map(|proof| zip_sum(v1, v2.retag(proof.rev())).into_inner()),
        Some(vec![2, 5, 7, 10])
    );
}