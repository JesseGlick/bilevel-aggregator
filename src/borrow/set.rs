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

    /// List the pairs currently in the collection without consuming
    /// the collection.
    /// 
    /// Pairs are grouped by g.
    pub fn iter(&self) -> Iter<'_, G, K> {
        Iter::new(self)
    }
}

impl<G, K> BilevelSet<G, K>
where
    G: Clone + PartialEq + Hash,
    K: Clone + PartialEq + Hash,
{
    /// Copy the data into a new collection that groups by the aggregation key.
    pub fn pivot(&self) -> BilevelSet<K, G> {
        let capacity = Capacity {
            groups: self.keys.len(),
            agg_keys: self.groups.len(),
            per_group: self.per_group,
        };
        let mut pivoted = BilevelSet::with_capacity(capacity);
        for (g, k) in self.iter() {
            pivoted.insert(k, g);
        }
        pivoted
    }
}

pub struct Iter<'a, G, K> {
    keys: &'a Vec<K>,
    outer: hashbrown::hash_table::Iter<'a, (G, HashSet<usize>)>,
    inner: Option<(&'a G, std::collections::hash_set::Iter<'a, usize>)>,
}

impl<'a, G, K> Iter<'a, G, K> {
    fn new(set: &'a BilevelSet<G, K>) -> Self {
        let mut outer = set.groups.iter();
        let inner = outer.next().map(wrap_inner);
        Self { keys: &set.keys, outer, inner }
    }
}

impl<'a, G, K> Iterator for Iter<'a, G, K> {
    type Item = (&'a G, &'a K);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner) = &mut self.inner {
                if let Some(&i) = inner.1.next() {
                    return Some((inner.0, &self.keys[i]));
                } else {
                    self.inner = self.outer.next().map(wrap_inner);
                }
            } else {
                return  None;
            }
        }
    }
}

fn wrap_inner<G>(inner: &(G, HashSet<usize>))
    -> (&G, std::collections::hash_set::Iter<'_, usize>)
{
    (&inner.0, inner.1.iter())
}