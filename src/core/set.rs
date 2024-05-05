use std::{hash::Hash, marker::PhantomData};
use super::Capacity;
pub struct BilevelSet<G: Hash, K: Hash> {
    g: PhantomData<G>,
    k: PhantomData<K>,
}

impl<G: Hash, K: Hash> BilevelSet<G, K>
{
    pub fn new() -> Self {
        todo!()
    }

    pub fn with_capacity(capacity: Capacity) -> Self {
        todo!()
    }

    pub fn insert(
        &mut self,
        g: impl ToOwned<Owned = G> + PartialEq<G>,
        k: impl ToOwned<Owned = K> + PartialEq<K>,
    ) -> bool {
        todo!()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&G, &K)> {
        todo!()
    }
}