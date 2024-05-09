use std::{collections::HashMap, hash::Hash};
use hashbrown::HashTable;

use crate::{Capacity, hash};

/// A collection of distinct pairs (g, k) grouped by g, with a payload
/// associated with each pair.
/// 
/// As pairs are found, they are added if not already present.
/// When the collection is iterated over, the pairs are listed by group.
/// 
/// G is the type of the group key.
/// K is the type of the remaining key.
/// V is the type of the payload.
pub struct BilevelMap<G, K, V> {
    per_group: usize,
    /// Keep a single copy of each key here, rather than one in each group
    /// where it appears.
    keys: Vec<K>,
    groups: HashTable<(G, HashMap<usize, V>)>,
    key_table: HashTable<usize>,
}

impl<G: Hash, K: Hash, V: Default> BilevelMap<G, K, V> {
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

    /// Get a mutable reference to the payload for the specified key pair.
    /// 
    /// If the key pair is currently not present, the default payload is inserted.
    pub fn add_or_get<GRef, KRef>(&mut self, g: &GRef, k: &KRef) -> &mut V
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
        self.groups.entry(
            hash(g),
            |(o, _)| g.eq(o),
            |(o, _)| hash(o)
        ).or_insert_with(||(g.to_owned(), HashMap::with_capacity(self.per_group)))
            .into_mut().1.entry(i)
            .or_insert_with(V::default)
    }

    /// List the payloads for the pairs currently in the collection,
    /// without consuming the collection or the payloads.
    /// 
    /// Pairs are grouped by g.
    pub fn iter(&self) -> Iter<'_, G, K, V> {
        Iter::new(self)
    }
}

pub struct Iter<'a, G, K, V> {
    keys: &'a Vec<K>,
    outer: hashbrown::hash_table::Iter<'a, (G, HashMap<usize, V>)>,
    inner: Option<(&'a G, std::collections::hash_map::Iter<'a, usize, V>)>,
}

impl<'a, G, K, V> Iter<'a, G, K, V> {
    fn new(map: &'a BilevelMap<G, K,V>) -> Self {
        let mut outer = map.groups.iter();
        let inner = outer.next().map(wrap_inner);
        Self { keys: &map.keys, outer, inner }
    }
}

impl<'a, G, K, V> Iterator for Iter<'a, G, K, V> {
    type Item = (&'a G, &'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner) = &mut self.inner {
                if let Some((&i, v)) = inner.1.next() {
                    return Some((inner.0, &self.keys[i], v));
                } else {
                    self.inner = self.outer.next().map(wrap_inner);
                }
            } else {
                return  None;
            }
        }
    }
}

fn wrap_inner<G, V>(inner: &(G, HashMap<usize, V>))
    -> (&G, std::collections::hash_map::Iter<'_, usize, V>)
{
    (&inner.0, inner.1.iter())
}