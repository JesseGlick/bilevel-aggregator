use std::{collections::HashSet, hash::Hash};
use hashbrown::HashTable;

use crate::{Capacity, hash};


/// A collection of distinct pairs (g, k) grouped by g.
/// 
/// As pairs are found, they are added if not already present.
/// When the collection is iterated over, the pairs are listed by group.
/// 
/// G is the type of the group key.
/// K is the type of the remaining key.
pub struct BilevelSet<G, K> {
    per_group: usize,
    /// Keep a single copy of each key here, rather than one in each group
    /// where it appears.
    keys: Vec<K>,
    groups: HashTable<(G, HashSet<usize>)>,
    key_table: HashTable<usize>,
}

impl<G: Hash, K: Hash> BilevelSet<G, K> {
    /// Create a new collection.
    /// 
    /// No initial capacity is allocated, and capacity for a few items
    /// is allocated for each new group key found.
    pub fn new() -> Self {
        Self {
            per_group: 4,
            keys: Vec::new(),
            groups: HashTable::new(),
            key_table: HashTable::new(),
        }
    }

    /// Create a new collection with the specified capacity.
    pub fn with_capacity(capacity: Capacity) -> Self {
        let Capacity { groups, per_group, agg_keys } = capacity;
        Self {
            per_group,
            keys: Vec::with_capacity(agg_keys),
            groups: HashTable::with_capacity(groups),
            key_table: HashTable::with_capacity(agg_keys),
        }
    }

    /// Insert a key pair found into the collection.
    /// 
    /// g: the group key.
    /// k: the remaining key.
    /// 
    /// Return false if the key was already present, otherwise true.
    pub fn insert<GRef, KRef>(&mut self, g: &GRef, k: &KRef) -> bool 
    where
        GRef: ToOwned<Owned = G> + PartialEq<G> + Hash + ?Sized,
        KRef: ToOwned<Owned = K> + PartialEq<K> + Hash + ?Sized,
    {
        // Find the index of k in the key list, 
        // adding it if it is new.
        let &i = self.key_table.entry(
            hash(&k),
            |&i| k.eq(&self.keys[i]),
            |&i| hash(&self.keys[i])
        ).or_insert_with(||{
            let i = self.keys.len();
            self.keys.push(k.to_owned());
            i
        }).get();
        // Add the index found to the group.
        self.groups.entry(
            hash(g),
            |(o, _)| g.eq(o),
            |(o, _)| hash(o)
        ).or_insert_with(|| (g.to_owned(), HashSet::with_capacity(self.per_group)))
        .get_mut().1.insert(i)
    }
}