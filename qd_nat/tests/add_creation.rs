use qd_nat::{with_n, Add, Nat};

#[test]
#[should_panic]
fn add_unknown() {
    with_n! {
        let _: Add<N, N> = Add::from_usize(0).unwrap();
    }
}

#[test]
#[should_panic]
fn add_mismatched() {
    with_n! {
        let _ = N::from_usize(1);
        let _: Add<N, N> = Add::from_usize(0).unwrap();
    }
}

#[test]
fn add_matched() {
    with_n! {
        let _ = N::from_usize(1);
        let _ = Add::<N, N>::from_usize(2).unwrap();
    }
}
