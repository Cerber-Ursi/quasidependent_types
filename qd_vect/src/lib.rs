use qd_core::*;
use qd_nat::*;
use std::marker::PhantomData;
use std::ops::Index;

#[derive(Clone)]
pub struct Vect<T, N: Nat>(Vec<T>, PhantomData<N>);
impl<Item, N: Nat> Dependent for Vect<Item, N> {
    type Native = Vec<Item>;
    type Frozen = [Item];
    fn freeze(&self) -> &Self::Frozen {
        self.0.as_slice()
    }
    fn freeze_mut(&mut self) -> &mut Self::Frozen {
        self.0.as_mut_slice()
    }
    fn into_native(self) -> Self::Native {
        self.0
    }
    fn as_native(&self) -> &Self::Native {
        &self.0
    }
}

pub fn collect<Item, N: Nat, N2: Nat + From<N>, I: IntoIterator<Item = Item>>(
    iter: I,
) -> (N, Vect<Item, N>) {
    let inner: Vec<_> = iter.into_iter().collect();
    (expect_nat(inner.len()), Vect(inner, PhantomData))
}

#[macro_export]
macro_rules! vect {
    ($data:expr) => {
        ::qd_nat::with_n! {
            let v = $data;
            $crate::collect::<_, _, N, _>(v)
        }
    };
}

impl<Item, N: Nat> Vect<Item, N> {
    pub fn retag<New: Nat>(self, _proof: Equiv<N, New>) -> Vect<Item, New> {
        Vect(self.0, PhantomData)
    }

    pub fn size_refl(&self) -> Equiv<N, N> {
        Equiv::refl()
    }

    pub fn push(mut self, item: Item) -> Vect<Item, Add<N, typenum::consts::U1>> {
        self.0.extend(std::iter::once(item));
        Vect(self.0, PhantomData)
    }

    pub fn find_index(&self, mut pred: impl FnMut(&Item) -> bool) -> Option<Fin<N>> {
        N::iter_until().find(|&n| pred(&self[n]))
    }

    pub fn concat<N2: Nat>(mut self, vect: Vect<Item, N2>) -> Vect<Item, Add<N, N2>> {
        self.0.extend(vect.into_native());
        Vect(self.0, PhantomData)
    }
}

impl<Item, N: Nat> Index<Fin<N>> for Vect<Item, N> {
    type Output = Item;
    fn index(&self, index: Fin<N>) -> &Item {
        unsafe { self.0.get_unchecked(index.as_usize()) }
    }
}
