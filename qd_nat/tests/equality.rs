use qd_nat::{Equiv, Nat};

fn accepts_pair<N1: Nat, N2: Nat>(n1: N1, n2: N2, _proof: Equiv<N1, N2>) {
    assert_eq!(n1.as_usize(), n2.as_usize()); // never fails
}

#[test]
fn test_pair() {
    use qd_nat::with_n;

    let n = with_n!(N::from_usize(1).unwrap());
    let n1 = with_n!(N::from_usize(1).unwrap());
    let n2 = with_n!(N::from_usize(2).unwrap());

    match Equiv::try_prove_for(n, n1) {
        Some(proof) => accepts_pair(n, n1, proof),
        None => panic!("1 != 1, WTF?"),
    };
    match Equiv::try_prove_for(n1, n2) {
        None => {}
        Some(_) => panic!("1 == 2, WTF?"),
    }
}
