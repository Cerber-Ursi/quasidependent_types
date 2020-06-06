use qd_nat::{Equiv, Nat, Add};

fn accepts_pair<N1: Nat, N2: Nat>(n1: N1, n2: N2, _proof: Equiv<N1, N2>) {
    assert_eq!(n1.as_usize(), n2.as_usize()); // never fails
}

#[test]
fn checked() {
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

#[test]
fn proved() {
    use qd_nat::with_n;
    use qd_core::StaticallyProvable;

    let n1 = with_n!(N::from_usize(1).unwrap());
    accepts_pair(n1, n1, Equiv::proof());

    let n2 = with_n!(N::from_usize(2).unwrap());

    accepts_pair(Add::sum(n1, n2), Add::sum(n1, n2), Equiv::proof());
    accepts_pair(Add::sum(n1, n2), Add::sum(n2, n1), Equiv::proof());

    let n3 = with_n!(N::from_usize(3).unwrap());
    accepts_pair(Add::sum(n1, Add::sum(n2, n3)), Add::sum(Add::sum(n1, n2), n3), Equiv::proof());
}

#[test]
fn typenum() {
    use qd_core::StaticallyProvable;
    use qd_nat::Add;
    use typenum::{U1, U2, U3};

    accepts_pair(Add::<U1, U2>::get(), U3::default(), Equiv::proof());
}
