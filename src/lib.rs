//! Aggregate groups of groups.
//! 
//! This crate provides structures to group and aggregate data by some
//! composite key value, and then group the aggregated results by some
//! component of that key. The unique key for each row of aggregated data
//! is called the *full key*, the key by which aggregated results are grouped
//! is called the *group key*, and that portion of the full key that does not
//! form part of the *group key* is called the *aggregation key*.
//! The group key and aggregation key can each be simple or composite.
//! The key values are assumed to be text.
//!
//! Two primary structures are provided: BilevelSet, which merely lists the
//! aggregation keys associated with each group key, and BilevelMap,
//! which allows a payload to be kept for each pairing.
//!
//! For illustration purposes, the trivial case where group and aggregation key
//! values are simple scalars rather than potentially composite text
//! is also implemented.

/// Implementations where both the group key and the aggregation key
/// are copy types.
pub mod copy {
    mod map;
    mod set;

    pub use map::BilevelMap;
    pub use set::BilevelSet;

    #[cfg(test)]
    pub mod tests;
}

/// The capacity dimensions of a BilateralSet of BilateralTree.
pub struct Capacity {
    /// The number of groups to allocate space for.
    pub groups: usize,
    /// The number of items to allocate capacity for when a new
    /// group key is found.
    pub per_group: usize,
    /// The number of distinct aggregator keys to allocate space for.
    pub agg_keys: usize,
}