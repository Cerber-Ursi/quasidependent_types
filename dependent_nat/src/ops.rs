
    use crate::Nat;
    use std::marker::PhantomData;

    pub struct Add<N1: Nat, N2: Nat>(PhantomData<(N1, N2)>);
    impl<N1: Nat, N2: Nat> Nat for Add<N1, N2> {

    }
