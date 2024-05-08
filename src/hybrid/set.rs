use std::{collections::{HashMap, HashSet}, hash::Hash};
use hashbrown::HashTable;

use crate::{Capacity, hash};

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
    /// Keep a single copy of each key here, rather than one in each group
    /// where it appears.
    keys: Vec<K>,
    groups: HashMap<G, HashSet<usize>>,
    key_table: HashTable<usize>,
}

impl<G, K> BilevelSet<G, K>
where
    G: Hash + Eq + Copy,
    K: Hash
{
    /// Create a new collection.
    /// 
    /// No initial capacity is allocated, and capacity for a few items
    /// is allocated for each new group key found.
    pub fn new() -> Self {
        Self {
            per_group: 4,
            keys: Vec::new(),
            groups: HashMap::new(),
            key_table: HashTable::new(),
        }
    }

    /// Create a new collection with the specified capacity.
    pub fn with_capacity(capacity: Capacity) -> Self {
        let Capacity { groups, per_group, agg_keys } = capacity;
        Self {
            per_group,
            keys: Vec::with_capacity(agg_keys),
            groups: HashMap::with_capacity(groups),
            key_table: HashTable::with_capacity(agg_keys),
        }
    }

    /// Insert a key pair found into the collection.
    /// 
    /// g: the group key.
    /// k: the remaining key.
    /// 
    /// Return false if the key was already present, otherwise true.
    pub fn insert(
        &mut self,
        g: G,
        k: impl ToOwned<Owned = K> + PartialEq<K> + Hash
    ) -> bool {
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
        self.groups.entry(g)
            .or_insert(HashSet::with_capacity(self.per_group))
            .insert(i)
    }

    /// List the pairs currently in the collection without consuming
    /// the collection.
    /// 
    /// Pairs are grouped by g.
    /// 
    /// Since G is a Copy type, owned values are returned for g.
    pub fn iter(&self) -> impl Iterator<Item = (G, &K)> {
        self.groups.iter()
            .flat_map(|(g, inner)| inner.iter().map(|i| (*g, &self.keys[*i])))
    }
}

