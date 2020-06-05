use qd_nat::{with_n, Fin, Nat, NatIterUntil};

#[test]
fn overflow() {
    let n = with_n! { N::from_usize(usize::MAX).unwrap() };
    let mut iter = n.iter_until_this().skip(usize::MAX - 1);
    assert_eq!(iter.next().map(Fin::as_usize), Some(usize::MAX - 1));
    assert_eq!(iter.next().map(Fin::as_usize), None);
    assert_eq!(iter.next().map(Fin::as_usize), None);
}