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
    G: Hash + Eq,
    K: Hash + Eq,
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

    pub fn iter<'a>(&'a self) -> Iter<'a, G, K> {
        Iter {
            outer: self.data.iter(),
            inner: None
        }
    }
}

pub struct Iter<'a, G, K> {
    outer: std::collections::hash_map::Iter<'a, G, HashSet<K>>,
    inner: Option<(&'a G, std::collections::hash_set::Iter<'a, K>)>,
}

impl<'a, G, K> Iterator for Iter<'a, G, K> {
    type Item = (&'a G, &'a K);

    fn next(&mut self) -> Option<Self::Item> {
        // Get the next value from the current inner loop, there is one.
        // Otherwise look for the next non-empty inner.
        if let Some((g, inner)) = &mut self.inner {
            if let Some(k) = inner.next() {
                Some((g, k))
            } else {
                self.search()
            }
        } else {
            self.search()
        }
    }
}

impl<'a, G, K> Iter<'a, G, K> {
    fn search(&mut self) -> Option<(&'a G, &'a K)> {
        // Look for the next non-empty inner loop.
        loop {
            if let Some((g, inner)) = self.outer.next() {
                let mut inner = inner.iter();
                if let Some(k) = inner.next() {
                    // A non-empty inner was found.
                    // Return the first item and make it the current one.
                    self.inner = (g, inner)
                }
            } else {
                // The outer loop has been exhausted.
                return  None;
            }
        }
    }
}