use qd_core::*;
use qd_nat::*;
use qd_vect::{vect, Vect};
use std::ops::Add;

fn zip_sum<T: Clone + Add<T, Output = T>, N: Nat>(
    first: Vect<T, N>,
    second: Vect<T, N>,
) -> Vect<T, N> {
    let mut v1 = first;
    let v2 = second.freeze();
    v1.freeze_mut()
        .iter_mut()
        .zip(v2.iter())
        .for_each(|(i1, i2)| *i1 = i1.clone() + i2.clone());
    v1
}

fn equiv<T: Clone + PartialEq<T>, N: Nat>(first: Vect<T, N>, second: Vect<T, N>) -> bool {
    first
        .freeze()
        .iter()
        .zip(second.freeze())
        .all(|(i1, i2)| i1 == i2)
}

#[test]
fn summing() {
    let (_, v1) = vect!(vec![1]);
    let (_, v2) = vect!(vec![1]);

    assert_eq!(
        Equiv::check()
            .map(|proof| v2.retag(proof))
            .map(|v2| zip_sum(v1, v2))
            .map(Vect::into_native),
        Some(vec![2])
    );
}

#[test]
fn summing_mismatch() {
    let (_, v1) = vect!(vec![]);
    let (_, v2) = vect!(vec![1]);

    assert_eq!(
        Equiv::check()
            .map(|proof| v2.retag(proof))
            .map(|v2| zip_sum(v1, v2))
            .map(Vect::into_native),
        None
    );
}

#[test]
fn assigning() {
    let (_, mut v1) = vect!(vec![1]);
    let (_, v2) = vect!(vec![2]);

    match Equiv::check() {
        Some(proof) => {
            assert_eq!(v1.into_native(), vec![1]);
            v1 = v2.retag(proof);
            assert_eq!(v1.into_native(), vec![2]);
        }
        None => panic!("1 != 1, WTF?"),
    };
}

#[test]
fn mismatch() {
    let (n1, _) = vect!(vec![1]);
    let (n2, _) = vect!(vec![2, 3]);
    assert!(Equiv::try_prove_for(n1, n2).is_none());
}

#[test]
fn runtime_size() {
    macro_rules! parse_i64_discarding {
        ($string:literal) => {
            vect! {{
                use std::str::FromStr;
                let string: &str = &*$string;
                string.split(",").map(i64::from_str).filter_map(Result::ok)
            }}
        };
    }

    let (_, v1) = parse_i64_discarding!("1,2,3,four,5");
    let (_, v2) = parse_i64_discarding!("1,two,3,4,5");
    assert_eq!(
        Equiv::check().map(|proof| zip_sum(v1.retag(proof), v2).into_native()),
        Some(vec![2, 5, 7, 10])
    );
}

#[test]
#[should_panic(expected = "Attempted to override already stored value 0 with 1")]
fn runtime_size_mismatch() {
    use qd_vect::collect;
    with_n! {
        let (_, mut v) = collect::<_, N, _>(vec![]);
        assert_eq!(v.as_native().len(), 0);
        v = collect(vec![1]).1;
        // this statement is in fact unreachable, since the previous one should panic
        assert_eq!(v.as_native().len(), 1);
    }
}

#[test]
fn fixed_size() {
    use qd_vect::collect;
    use typenum::consts::*;
    let (_, v) = collect::<_, U2, _>(vec![1u32, 2]);
    assert_eq!(v.into_native(), vec![1u32, 2]);
}

#[test]
#[should_panic(expected = "Attempted to override already stored value 2 with 1")]
fn fixed_size_mismatch() {
    use qd_vect::collect;
    use typenum::consts::*;
    let _ = collect::<_, U2, _>(&[1]);
}

#[test]
fn pushing() {
    let (_, v) = vect!(vec![1]);
    let v = v.push(2);
    assert_eq!(v.into_native(), vec![1, 2]);
}

#[test]
fn get_fin() {
    let (_, v) = vect!(vec![1]);
    let index = Fin::from_usize(0).expect("0 >= 1, WTF?");
    assert_eq!(v[index], 1);
}

#[test]
fn find() {
    let (_, v) = vect!(vec![10, 20, 30]);

    let n = v.find_index(|&n| n == 10);
    assert_eq!(n.map(Fin::as_usize), Some(0));
    assert_eq!(v[n.unwrap()], 10);

    let n = v.find_index(|&n| n == 20);
    assert_eq!(n.map(Fin::as_usize), Some(1));
    assert_eq!(v[n.unwrap()], 20);

    assert_eq!(v.find_index(|&n| n == 0).map(Fin::as_usize), None);
}

#[test]
fn concats() {
    let (_, v1) = vect!(vec![1]);
    let (n2, v2) = vect!(vec![2, 3]);

    let v12 = v1.clone().concat(v2.clone());
    let v21 = v2.clone().concat(v1.clone());

    let sum = zip_sum(v12, v21.retag(Equiv::proof()));
    assert_eq!(sum.into_native(), vec![3, 5, 4]);

    let (n3, v3) = vect!(vec![4, 5]);
    let v12 = v1.clone().concat(v2.clone());
    let v13 = v1.clone().concat(v3.clone());

    match Equiv::try_prove_for(n2, n3) {
        Some(proof) => {
            let sum = zip_sum(v12.retag(Equiv::deduce(proof)), v13);
            assert_eq!(sum.into_native(), vec![2, 6, 8]);
        }
        None => panic!("2 != 2, WTF?"),
    }

    let v_12_3 = v1.clone().concat(v2.clone()).concat(v3.clone());
    let v_1_23 = v1.concat(v2.concat(v3));
    assert!(equiv(v_1_23.retag(Equiv::proof()), v_12_3));
}
