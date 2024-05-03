//! Versions of BilevelMap and BilevelSet using scalar keys.

use std::{collections::{HashMap, HashSet}, hash::Hash};

/// A collection of distinct pairs (g, k) grouped by g.
/// 
/// As pairs are found, they are added if not already present.
/// When the collection is iterated over, the pairs are listed by group.
/// 
/// G is the type of the group key.
/// K is the type of the remaining key.
pub struct BilevelSet<G, K>
where
G: Hash + Eq + Copy + 'static,
K: Hash + Eq + Copy,
{
    data: HashMap<G, HashSet<K>>,
    per_group: usize,
}

impl<G, K> BilevelSet<G, K>
where
    G: Hash + Eq + Copy + 'static,
    K: Hash + Eq + Copy,
{
    /// Create a new collection.
    /// 
    /// No initial capacity is allocated, and capacity for a few items
    /// is allocated for each new group key found.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            per_group: 4,
        }
    }

    /// Create a new collection with the specified capacity.
    /// 
    /// groups: The number of groups to allocate space for.
    /// per_group: The number of items to allocate capacity for when a new
    ///     group key is found.
    pub fn with_capacity(groups: usize, per_group: usize) -> Self {
        Self {
            data: HashMap::with_capacity(groups),
            per_group
        }
    }

    /// Insert a key pair found into the collection.
    /// 
    /// g: the group key.
    /// k: the remaining key.
    /// 
    /// Return true if the key was already present.
    pub fn insert(&mut self, g: G, k: K) -> bool {
        self.data.entry(g)
            .or_insert(HashSet::with_capacity(self.per_group))
            .insert(k)
    }

    /// List the pairs currently in the collection without consuming
    /// the collection.
    /// 
    /// Pairs are grouped by g.
    /// 
    /// Since G and K are Copy types, owned values are returned.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (G, K)> + 'a {
        self.data.iter()
            .map(|(g, inner)| inner.iter().map(|k| (*g, *k)))
            .flatten()
    }

    /// List the pairs in the collection and consume the collection.
    /// 
    /// Pairs are grouped by g.
    pub fn into_iter(self) -> impl Iterator<Item = (G, K)>{
        self.data.into_iter()
            .map(|(g, inner)| inner.into_iter().map(move |k| (g, k)))
            .flatten()
    }
}

/// A collection of distinct pairs (g, k) grouped by g, with a payload
/// associated with each pair.
/// 
/// As pairs are found, they are added if not already present.
/// When the collection is iterated over, the pairs are listed by group.
/// 
/// G is the type of the group key.
/// K is the type of the remaining key.
/// V is the type of the payload.
pub struct BilevelMap <G, K, V>
where
    G: Hash + Eq + Copy + 'static,
    K: Hash + Eq + Copy,
{
    data: HashMap<G, HashMap<K, V>>,
    per_group: usize,
    constructor: Box<dyn Fn () -> V>,
}

impl<G, K, V> BilevelMap<G, K, V> 
where
    G: Hash + Eq + Copy + 'static,
    K: Hash + Eq + Copy,
{
    /// Create a new collection.
    /// 
    /// No initial capacity is allocated, and capacity for a few items
    /// is allocated for each new group key found.
    /// 
    /// constructor: A constructor for the payload.
    pub fn new(constructor: impl Fn () -> V + 'static) -> Self {
        Self {
            data: HashMap::new(),
            per_group: 4,
            constructor: Box::new(constructor)
        }
    }

    /// Create a new collection with the specified capacity.
    /// 
    /// groups: The number of groups to allocate space for.
    /// per_group: The number of items to allocate capacity for when a new
    ///     group key is found.
    /// constructor: A constructor for the payload.
    pub fn with_capacity(
        groups: usize,
        per_group: usize,
        constructor: impl Fn () -> V + 'static
    ) -> Self {
        Self {
            data: HashMap::with_capacity(groups),
            per_group,
            constructor: Box::new(constructor)
        }
    }

    /// Get a mutable reference to the payload for the specified key pair.
    /// 
    /// If the key pair is currently not present, the default payload is inserted.
    pub fn add_or_get(&mut self, g: G, k: K) -> &mut V {
        self.data.entry(g)
            .or_insert(HashMap::with_capacity(self.per_group))
            .entry(k)
            .or_insert_with(&self.constructor)

    }

    /// List the payloads for the pairs currently in the collection,
    /// without consuming the collection or the payloads.
    /// 
    /// Pairs are grouped by g.
    /// 
    /// Since G and K are copy types, owned keys are returned, but the payload
    /// is still returned by reference.
    pub fn iter(&self) -> impl Iterator<Item = (G, K, &V)> {
        self.data.iter()
            .map(|(g, inner)| inner.iter().map(|(k, v)| (*g, *k, v)))
            .flatten()
    }

    /// List and consume the payloads for the pairs in the collection,
    /// consuming the collection.
    /// 
    /// Pairs are grouped by g.
    pub fn into_iter(self) -> impl Iterator<Item = (G, K, V)> {
        self.data.into_iter()
            .map(|(g, inner)| inner.into_iter().map(move |(k, v)| (g, k, v)))
            .flatten()
    }
}