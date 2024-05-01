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
    G: Hash + Eq,
    K: Hash + Eq,
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
    pub fn insert(&mut self, g: G, k: K) -> bool {
        self.data.entry(g)
            .or_insert(HashSet::with_capacity(self.per_group))
            .insert(k)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (G, K)> + 'a {
        self.data.iter()
            .map(|(g, set)| set.iter().map(|k| (*g, *k)))
            .flatten()
    }

    pub fn into_iter(self) -> impl Iterator<Item = (G, K)>{
        self.data.into_iter()
            .map(|(g, set)| set.into_iter().map(move |k| (g, k)))
            .flatten()
    }
}