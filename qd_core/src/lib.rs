//! Example crate for the simulation of dependent typing.

/// Main trait, describing possible dependent type.
pub trait Dependent {
    type Native;
    type Frozen: ?Sized;
    fn freeze(&self) -> &Self::Frozen;
    fn freeze_mut(&mut self) -> &mut Self::Frozen;
    fn into_native(self) -> Self::Native;
    fn as_native(&self) -> &Self::Native;
}

pub trait StaticallyProvable<Rule>
where
    Self: Sized,
{
    fn proof() -> Self;
    fn prove_by_rule(_: Rule) -> Self {
        Self::proof()
    }
}

pub trait Deducible<Reason> {
    fn deduce(_: Reason) -> Self;
}

impl<Rule, T: StaticallyProvable<Rule>> Deducible<Rule> for T {
    fn deduce(_: Rule) -> Self {
        Self::proof()
    }
}
