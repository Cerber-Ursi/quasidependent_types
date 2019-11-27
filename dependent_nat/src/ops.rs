pub(crate) trait NatOp {}

pub mod structs {
    use crate::Nat;
    use std::marker::PhantomData;
    use super::NatOp;

    pub struct Add<N1: Nat, N2: Nat>(PhantomData<(N1, N2)>);
    impl<N1: Nat, N2: Nat> NatOp for Add<N1, N2> {}
    impl<N1: Nat, N2: Nat> Nat for Add<N1, N2> {

    }
}
