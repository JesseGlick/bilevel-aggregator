use crate::core::Capacity;

/// A collection of distinct pairs (g, k) grouped by g, with a payload
/// associated with each pair.
/// 
/// As pairs are found, they are added if not already present.
/// When the collection is iterated over, the pairs are listed by group.
/// 
/// All keys are arrays of strings.
/// G is the array size of the group key.
/// K is the array size of the remaining key.
/// V is the type of the payload.
pub struct BilevelMap<const G: usize, const K: usize, V> {
}

impl<const G: usize, const K: usize, V> BilevelMap<G, K, V> {
    /// Create a new collection.
    /// 
    /// No initial capacity is allocated, and capacity for a few items
    /// is allocated for each new group key found.
    /// 
    /// constructor: A constructor for the payload.
    pub fn new(constructor: impl Fn () -> V + 'static) -> Self {
        todo!()
    }

    /// Create a new collection with the specified capacity.
    /// 
    /// constructor: A constructor for the payload.
    pub fn with_capacity(
        capacity: Capacity,
        constructor: impl Fn () -> V + 'static
    ) -> Self {
        todo!()
    }

    /// Get a mutable reference to the payload for the specified key pair.
    /// 
    /// If the key pair is currently not present, the default payload is inserted.
    pub fn add_or_get(&mut self, g: [&str; G], k: [&str; K]) -> &mut V {
        todo!()
    }

    /// List the pairs currently in the collection with their payload
    /// without consuming the collection.
    /// 
    /// Pairs are grouped by g.
    pub fn iter(&self) -> impl Iterator<Item = ([&str; G], [&str; K], &V)> {
        todo!()
    }
}