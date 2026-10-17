use super::*;
use proptest::prop_assert_eq;

// #[derive(Debug)]
// struct BinarySegmentTree<Key, M>
// where
//     M: Monoid,
// {
//     root: Option<Rc<BinarySegmentTreeItem<Key, M>>>,
// }
//
// impl<Key, M> Clone for BinarySegmentTree<Key, M>
// where
//     M: Monoid,
// {
//     fn clone(&self) -> Self {
//         Self {
//             root: self.root.clone(),
//         }
//     }
// }
//
// #[derive(Debug)]
// enum BinarySegmentTreeItem<Key, M>
// where
//     M: Monoid,
// {
//     Leaf(Key, M::S),
//     Node {
//         left: Rc<BinarySegmentTreeItem<Key, M>>,
//         mid: Rc<(Key, M::S, u32)>,
//         right: Rc<BinarySegmentTreeItem<Key, M>>,
//     },
// }
//
// impl<Key, M> BinarySegmentTree<Key, M>
// where
//     Key: Clone + Ord,
//     M: Monoid,
// {
//     fn new() -> BinarySegmentTree<Key, M> {
//         BinarySegmentTree { root: None }
//     }
//
//     fn insert(&mut self, key: Key, value: M::S) {
//         let root = match &mut self.root {
//             None => {
//                 self.root = Some(Rc::new(BinarySegmentTreeItem::Leaf(key, value)));
//                 return;
//             }
//             Some(root) => root,
//         };
//         fn insert_inner<Key, M>(node: &mut Rc<BinarySegmentTreeItem<Key, M>>, key: Key, value: M::S)
//         where
//             Key: Clone + Ord,
//             M: Monoid,
//         {
//
//         }
//     }
//
//     fn remove(&mut self, key: &Key) -> bool {}
//
//     fn get(&self, key: &Key) -> Option<&M::S> {
//         let mut node = self.root.as_ref()?;
//         loop {
//             match &**node {
//                 BinarySegmentTreeItem::Leaf(k, v) => {
//                     return (k == key).then_some(v);
//                 }
//                 BinarySegmentTreeItem::Node { left, mid, right } => {
//                     node = if key < &mid.0 { left } else { right };
//                 }
//             }
//         }
//     }
//
//     fn sum(&self, range: impl RangeBounds<Key>) -> M::S {}
// }

#[test]
fn test_binary_search() {
    let array = vec![1, 2, 3, 4, 5];
    let f = |x: usize| array[x].cmp(&3);
    let result = binary_search(0..array.len(), f);
    let expected = array.binary_search(&3);
    assert_eq!(result, expected);
}

proptest::proptest! {
    #[test]
    fn test_binary_search_prop(mut array in proptest::collection::vec(-100i32..100, 0..100), target in 0..100) {
        array.sort_unstable();
        let f = |x: usize| array[x].cmp(&target);
        let result = binary_search(0..array.len(), f);
        let expected = array.binary_search(&target);
        prop_assert_eq!(result.is_ok(), expected.is_ok());
        match (result, expected) {
            (Err(result), Err(expected)) => {
                prop_assert_eq!(result, expected);
            }
            (Ok(result), _) => {
                prop_assert_eq!(array[result], target);
            }
            _ => {}
        }
    }
}

#[test]
fn test_all_bits_basic() {
    let expected_results = vec![
        vec![(0, false), (1, false), (2, false), (3, false)],
        vec![(0, true), (1, false), (2, false), (3, false)],
        vec![(0, false), (1, true), (2, false), (3, false)],
        vec![(0, true), (1, true), (2, false), (3, false)],
        vec![(0, false), (1, false), (2, true), (3, false)],
        vec![(0, true), (1, false), (2, true), (3, false)],
    ];

    all_bits().zip(expected_results).for_each(|(bits, expected)| {
        bits.zip(expected).for_each(|(bit, expected)| {
            assert_eq!(bit, expected);
        })
    });
}

#[test]
fn test_all_bits_large_value() {
    // This will test some larger numbers, specifically the binary representation of 255.
    let mut all_bits_iter = all_bits().skip(255).take(1);

    if let Some(bits) = all_bits_iter.next() {
        let results: Vec<(usize, bool)> = bits.take(8).collect(); // Check the first 8 bits.
        let expected = vec![
            (0, true), // LSB (rightmost bit)
            (1, true),
            (2, true),
            (3, true),
            (4, true),
            (5, true),
            (6, true),
            (7, true), // MSB (leftmost bit in 8-bit range, true for 255)
        ];
        assert_eq!(results, expected, "Bits for 255 did not match");
    } else {
        panic!("Expected 255 to be present in all_bits output");
    }
}
