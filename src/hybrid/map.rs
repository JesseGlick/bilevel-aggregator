use std::{collections::HashMap, hash::Hash};
use hashbrown::HashTable;
use crate::{Capacity, hash};

pub struct BilevelMap<G, K, V>
where
    G: Hash + Eq
{
    per_group: usize,
    /// Keep a single copy of each key here, rather than one in each group
    /// where it appears.
    keys: Vec<K>,
    groups: HashMap<G, HashMap<usize, V>>,
    key_table: HashTable<usize>,
}

impl<G, K, V> BilevelMap<G, K, V>
where
    G: Hash + Eq + Copy,
    K: Hash,
    V: Default,
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

    // Create a new collection with the specified capacity.
    pub fn with_capacity(capacity: Capacity) -> Self {
        let Capacity { groups, per_group, agg_keys } = capacity;
        Self {
            per_group,
            keys: Vec::with_capacity(agg_keys),
            groups: HashMap::with_capacity(groups),
            key_table: HashTable::with_capacity(agg_keys),
        }
    }

    /// Get a mutable reference to the payload for the specified key pair.
    /// 
    /// If the key pair is currently not present, the default payload is inserted.
    pub fn add_or_get<KRef>(&mut self, g: G, k: &KRef) -> &mut V
    where
        KRef: ToOwned<Owned = K> + PartialEq<K> + Hash + ?Sized
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
        self.groups.entry(g)
            .or_insert(HashMap::with_capacity(self.per_group))
            .entry(i)
            .or_insert_with(V::default)
    }

    /// List the payloads for the pairs currently in the collection,
    /// without consuming the collection or the payloads.
    /// 
    /// Pairs are grouped by g.
    /// 
    /// Since G is a Copy type, owned values are returned for g.
    pub fn iter(&self) -> impl Iterator<Item = (G, &K, &V)> {
        self.groups.iter()
            .flat_map(|(g, inner)| inner.iter().map(|(&i, v)| (*g, &self.keys[i], v)))
    }
}