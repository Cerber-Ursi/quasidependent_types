
use dependent::traits::{Dependent, Equiv, Nat, NatEq};
use dependent::vect::{collect, Vect};

fn zip_use_second<T: Clone, N1: Nat, N2: Nat>(
    first: Vect<T, N1>,
    second: Vect<T, N2>,
    _proof: Equiv<N1, N2>,
) -> Vect<T, N1> {
    first.map_ref(|v1| {
        second.consume_ref(|v2| {
            v1.iter_mut()
                .zip(v2.iter())
                .for_each(|(i1, i2)| *i1 = i2.clone())
        })
    })
}

macro_rules! vect {
    ($v:expr => $n:ident) => {{
        struct $n(usize);
        impl Nat for $n {
            fn as_usize(&self) -> usize {
                self.0
            }
            fn from_usize(s: usize) -> Self {
                Self(s)
            }
        }
        collect::<_, $n, _>($v)
    }}
}

#[test]
fn equal() {
    let (n1, v1) = vect!(vec![0] => N1);
    let (n2, v2) = vect!(vec![1] => N2);

    assert_eq!(
        NatEq::eq(&n1, &n2).map(|proof| zip_use_second(v1, v2, proof).into_inner()),
        Some(vec![1])
    );
}
