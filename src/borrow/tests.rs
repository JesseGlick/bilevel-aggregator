use std::collections::HashSet;

use super::*;
use crate::Capacity;

#[test]
pub fn test_set() {
    let test_data = [
        ("2", "2"),
        ("2", "4"),
        ("2", "8"),
        ("2", "10"),
        ("3", "3"),
        ("3", "3"),
        ("3", "6"),
        ("3", "9"),
        ("4", "4"),
        ("4", "8"),
        ("5", "5"),
        ("5", "5"),
        ("5", "10"),
        ];
    // Create tests with and without pre-allocated capacity.
    let mut a: BilevelSet<String, String> = BilevelSet::new();
    let mut b: BilevelSet<String, String> = BilevelSet::with_capacity(Capacity{
        groups: 4,
        per_group: 4,
        agg_keys: 8,
    });
    for (i, (g, k)) in test_data.iter().enumerate() {
        let in_a = a.insert(*g, *k);
        let in_b = b.insert(*g, *k);
        // Verify that insertion returns false on duplicates. otherwise true.
        let expected = match i {
            5 => false,
            11 => false,
            _ => true,
        };
        assert_eq!(in_a, expected);
        assert_eq!(in_b, expected);
    }
    // Collect the results both with and without consuming.
    let results: [Vec<_>; 2] = [
        a.iter().collect(),
        b.iter().collect(),
    ];
    for result in results {
        // Verify size of the results is the number of distinct pairs.
        assert_eq!(result.len(), 11);
        // Verify the presence of each pair.
        // If each pair expected is present, and the number of pairs is correct,
        // The list of pairs must be correct.
        for i in test_data.iter() {
            assert!(result.iter().any(|r| (r.0 == i.0) && (r.1 == i.1)))
        }
        // Verify that the results are grouped by the group key.
        // This means that once a group key appears, followed by a
        // different key, it must not appear again.
        let mut set: HashSet<String> = HashSet::new();
        let mut prev = "".to_owned();
        for (g, _) in result.into_iter() {
            if g != &prev {
                set.insert(prev);
                prev = g.to_owned();
            }
            assert!(!set.contains(g));
        }
    }
}

#[test]
pub fn test_map() {
    let test_data = [
        ("2", "2"),
        ("2", "4"),
        ("2", "8"),
        ("2", "10"),
        ("3", "3"),
        ("3", "3"),
        ("3", "6"),
        ("3", "9"),
        ("4", "4"),
        ("4", "8"),
        ("5", "5"),
        ("5", "5"),
        ("5", "10"),
        ];
    // Create tests with and without pre-allocated capacity.
    let mut a: BilevelMap<String, String, u32> = BilevelMap::new();
    let mut b = BilevelMap::with_capacity(
        Capacity{
            groups: 4,
            per_group: 4,
            agg_keys: 8,
        });
    for (i, (g, k)) in test_data.iter().enumerate() {
        let in_a = a.add_or_get(*g, *k);
        let in_b = b.add_or_get(*g, *k);
        // Verify that insertion returns a count of 1 on duplicates, and 0 otherwise.
        let expected: u32 = match i {
            5 => 1,
            11 => 1,
            _ => 0
        };
        assert_eq!(*in_a, expected);
        assert_eq!(*in_b, expected);
        // Increment the counters.
        *in_a += 1;
        *in_b += 1;
    }
    // Collect the results both with and without consuming.
    let results: [Vec<_>; 2] = [
        a.iter().collect(),
        b.iter().collect(),
    ];
    for result in results {
        // Verify size of the results is the number of distinct pairs.
        assert_eq!(result.len(), 11);
        // Verify the presence of each pair.
        // If each pair expected is present, and the number of pairs is correct,
        // The list of pairs must be correct.
        for &(ig, ik) in test_data.iter() {
            assert!(result.iter().any(|&(rg, rk, _)| (rg == ig) && (rk == ik)))
        }
        // Verify that each pair is associated with the correct count.
        for &(g, k, v) in result.iter() {
            let expected: u32 = if
                ((g == "3") && (k == "3")) || ((g == "5") && (k == "5"))
            {
                2
            } else {
                1
            };
            assert_eq!(*v, expected)
        }
        // Verify that the results are grouped by the group key.
        // This means that once a group key appears, followed by a
        // different key, it must not appear again.
        let mut set: HashSet<String> = HashSet::new();
        let mut prev = "".to_owned();
        for (g, _, _) in result.into_iter() {
            if g != &prev {
                set.insert(prev);
                prev = g.to_owned();
            }
            assert!(!set.contains(g));
        }
    }
}