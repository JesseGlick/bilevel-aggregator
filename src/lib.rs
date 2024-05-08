//! Aggregate groups of groups.
//! 
//! This crate provides structures to group and aggregate data by some
//! composite key value, and then group the aggregated results by some
//! component of that key. The unique key for each row of aggregated data
//! is called the *full key*, the key by which aggregated results are grouped
//! is called the *group key*, and that portion of the full key that does not
//! form part of the *group key* is called the *aggregation key*.
//! 
//! The group key and aggregation key can each be simple or composite.
//!
//! Two primary structures are provided: BilevelSet, which merely lists the
//! aggregation keys associated with each group key, and BilevelMap, which
//! allows a payload to be kept for each pairing.
//!
//! # Modules
//! 
//! - copy  Use the versions in this module where both the group key and the
//!         aggregation key are copy types.
//! - borrow Use the versions in this module where the group key is a copy
//!         type but the aggregation key is not.
//! - hybrid Use the versions in this module where neither key is a copy type.  


use std::hash::{Hash, Hasher, DefaultHasher};

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

/// Implementations where the group key is a copy type but the
/// aggregation key is not.
pub mod hybrid {
    mod map;
    mod set;

    pub use map::BilevelMap;
    pub use set::BilevelSet;

    #[cfg(test)]
    pub mod tests;
}

/// Implementations where neither the group key nor the aggregation key is
/// a copy type.
pub mod borrow {
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

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}