use super::*;

#[test]
pub fn test_set() {
    let test_data = [
        (2, 2),
        (2, 4),
        (2, 8),
        (2, 10),
        (3, 3),
        (3, 3),
        (3, 6),
        (3, 9),
        (4, 4),
        (4, 8),
        (5, 5),
        (5, 5),
        (5, 10),
        ];
    let mut a = BilevelSet::new();
    let mut b = BilevelSet::with_capacity(4, 4);
    for (i, (g, k)) in test_data.into_iter().enumerate() {
        let in_a = a.insert(g, k);
        let in_b = b.insert(g, k);
        let expected = match i {
            5 => false,
            11 => false,
            _ => true,
        };
        assert_eq!(in_a, expected);
        assert_eq!(in_b, expected);
    }
}