use crate::core::Capacity;

/// A collection of distinct pairs (g, k) grouped by g.
/// 
/// As pairs are found, they are added if not already present.
/// When the collection is iterated over, the pairs are listed by group.
/// 
/// All keys are arrays of strings.
/// G is the array size of the group key.
/// K is the array size of the remaining key.
pub struct BilevelSet<const G: usize, const K: usize> {

}

impl<const G: usize, const K: usize> BilevelSet<G, K> {
    /// Create a new collection.
    /// 
    /// No initial capacity is allocated, and capacity for a few items
    /// is allocated for each new group key found.
    pub fn new() -> Self{
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
    /// Return true if the key was already present.
    pub fn insert(&mut self, g: [&str; G], k: [&str; K]) -> bool {
        todo!()
    }

    /// List the pairs currently in the collection without consuming
    /// the collection.
    /// 
    /// Pairs are grouped by g.
    pub fn iter(&self) -> impl Iterator<Item = ([&str; G], [&str; K])> {
        todo!()
    }
}