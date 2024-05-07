use std::{collections::{HashMap, HashSet}, hash::Hash};
use hashbrown::HashTable;

use crate::Capacity;

/// A collection of distinct pairs (g, k) grouped by g.
/// 
/// As pairs are found, they are added if not already present.
/// When the collection is iterated over, the pairs are listed by group.
/// 
/// G is the type of the group key.
/// K is the type of the remaining key.
pub struct BilevelSet<G, K>
where
    G: Hash + Eq,
{
    per_group: usize,
    keys: Vec<K>,
    groups: HashMap<G, HashSet<usize>>,
    key_table: HashTable<usize>,
}

impl<G, K> BilevelSet<G, K>
where
    G: Hash + Eq + Copy
{
    /// Create a new collection.
    /// 
    /// No initial capacity is allocated, and capacity for a few items
    /// is allocated for each new group key found.
    pub fn new() -> Self {
        todo!()
    }

    /// Create a new collection with the specified capacity.
    pub fn with_capacity(capacity: Capacity) -> Self {
        todo!()
    }

    /// Insert a key pair found into the collection.
    /// 
    /// g: the group key.
    /// k: the remaining key.
    /// 
    /// Return false if the key was already present, otherwise true.
    pub fn insert(
        &self,
        g: G,
        k: impl ToOwned<Owned = K> + PartialEq<K> + Hash
    ) -> bool {
        todo!()
    }

    /// List the pairs currently in the collection without consuming
    /// the collection.
    /// 
    /// Pairs are grouped by g.
    /// 
    /// Since G is a Copy type, owned values are returned for g.
    pub fn iter(&self) -> impl Iterator<Item = (G, &K)> {
        todo!()
    }
}