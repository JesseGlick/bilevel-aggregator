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
/// 
/// # Examples
/// ```
/// use bilevel_aggregator::copy::BilevelSet;
/// 
/// let mut set = BilevelSet::new();
/// set.insert(1, 2);
/// set.insert(2, 1);
/// set.insert(1, 2);
/// set.insert(2, 2);
/// for (g, k) in set.into_iter() {
///     println!("{}, {}", g, k)
/// }
/// ```
/// The results will be de-duplicated, and grouped by g.
/// For example:
/// 
/// 1, 2
/// 2, 1
/// 2, 2
/// 
/// ```
/// use bilevel_aggregator::copy::BilevelMap;
/// 
/// let mut map = BilevelMap::<usize, usize, usize>::new();
/// *map.add_or_get(1, 2) += 1;
/// *map.add_or_get(2, 1) += 1;
/// *map.add_or_get(1, 2) += 1;
/// *map.add_or_get(2, 2) += 1;
/// for (g, k, v) in map.into_iter() {
///     println!("{}, {}, {}", g, k, v)
/// }
/// ```
/// The results will be grouped by g.
/// For example:
/// 
/// 1, 2, 2
/// 2, 1, 1
/// 2, 2, 1
/// 
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
/// 
/// # Examples
/// ```
/// use bilevel_aggregator::hybrid::BilevelSet;
/// 
/// let mut set = BilevelSet::new();
/// set.insert(1, "2");
/// set.insert(2, "1");
/// set.insert(1, "2");
/// set.insert(2, "2");
/// for (g, k) in set.iter() {
///     println!("{}, {}", g, k)
/// }
/// ```
/// The results will be de-duplicated, and grouped by g.
/// For example:
/// 
/// 1, 2
/// 2, 1
/// 2, 2
/// 
/// ```
/// use bilevel_aggregator::hybrid::BilevelMap;
/// 
/// let mut map = BilevelMap::<usize, String, usize>::new();
/// *map.add_or_get(1, "2") += 1;
/// *map.add_or_get(2, "1") += 1;
/// *map.add_or_get(1, "2") += 1;
/// *map.add_or_get(2, "2") += 1;
/// for (g, k, v) in map.iter() {
///     println!("{}, {}, {}", g, k, v)
/// }
/// ```
/// The results will be grouped by g.
/// For example:
/// 
/// 1, 2, 2
/// 2, 1, 1
/// 2, 2, 1
/// 
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
/// 
/// # Examples
/// ```
/// use bilevel_aggregator::borrow::BilevelSet;
/// 
/// let mut set = BilevelSet::new();
/// set.insert("1", "2");
/// set.insert("2", "1");
/// set.insert("1", "2");
/// set.insert("2", "2");
/// for (g, k) in set.iter() {
///     println!("{}, {}", g, k)
/// }
/// ```
/// The results will be de-duplicated, and grouped by g.
/// For example:
/// 
/// 1, 2
/// 2, 1
/// 2, 2
/// 
/// ```
/// use bilevel_aggregator::borrow::BilevelMap;
/// 
/// let mut map = BilevelMap::<String, String, usize>::new();
/// *map.add_or_get("1", "2") += 1;
/// *map.add_or_get("2", "1") += 1;
/// *map.add_or_get("1", "2") += 1;
/// *map.add_or_get("2", "2") += 1;
/// for (g, k, v) in map.iter() {
///     println!("{}, {}, {}", g, k, v)
/// }
/// ```
/// The results will be grouped by g.
/// For example:
/// 
/// 1, 2, 2
/// 2, 1, 1
/// 2, 2, 1
/// 
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

fn hash<T: Hash + ?Sized>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}