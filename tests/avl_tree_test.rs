extern crate binary_search_tree;
use binary_search_tree::{
    avl_tree::{AVLTree},
};

#[test]
fn into_iter_test_empty() {
    let tree: AVLTree<i32, i32> = AVLTree::new();
    let mut iter = tree.into_iter();
    assert_eq!(iter.next(), None);
}

#[test]
fn insert_test() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(
        tree.into_iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>(),
        vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]
    );
}

#[test]
fn insert_test_confused() {
    let mut tree = AVLTree::new();
    tree.insert(4, 4);
    tree.insert(1, 1);
    tree.insert(3, 3);
    tree.insert(2, 2);
    tree.insert(5, 5);
    assert_eq!(
        tree.into_iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>(),
        vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]
    );
}

#[test]
fn insert_test_balanced() {
    let mut tree = AVLTree::new();
    tree.insert(4, 4);
    tree.insert(2, 2);
    tree.insert(6, 6);
    tree.insert(1, 1);
    tree.insert(3, 3);
    tree.insert(5, 5);
    tree.insert(7, 7);
    assert_eq!(
        tree.into_iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>(),
        vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)]
    );
}

#[test]
fn remove_test() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.remove(&3), Some(3));
    assert_eq!(
        tree.into_iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>(),
        vec![(1, 1), (2, 2), (4, 4), (5, 5)]
    );
}

#[test]
fn remove_test_all() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.remove(&1), Some(1));
    assert_eq!(tree.remove(&2), Some(2));
    assert_eq!(tree.remove(&3), Some(3));
    assert_eq!(tree.remove(&4), Some(4));
    assert_eq!(tree.remove(&5), Some(5));
    assert!(tree.is_empty())
}

#[test]
fn remove_test_confused() {
    let mut tree = AVLTree::new();
    tree.insert(4, 4);
    tree.insert(1, 1);
    tree.insert(3, 3);
    tree.insert(2, 2);
    tree.insert(5, 5);
    assert_eq!(tree.remove(&4), Some(4));
    assert_eq!(
        tree.into_iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>(),
        vec![(1, 1), (2, 2), (3, 3), (5, 5)]
    );
}

#[test]
fn remove_test_balanced() {
    let mut tree = AVLTree::new();
    tree.insert(4, 4);
    tree.insert(2, 2);
    tree.insert(6, 6);
    tree.insert(1, 1);
    tree.insert(3, 3);
    tree.insert(5, 5);
    tree.insert(7, 7);
    assert_eq!(tree.remove(&4), Some(4));
    assert_eq!(
        tree.into_iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>(),
        vec![(1, 1), (2, 2), (3, 3), (5, 5), (6, 6), (7, 7)]
    );
}

#[test]
fn remove_test_not_exists() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.remove(&6), None);
    assert_eq!(
        tree.into_iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>(),
        vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)]
    );
}

#[test]
fn get_test() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.get(&4), Some(&4));
}

#[test]
fn get_test_not_exists() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.get(&6), None);
}

#[test]
fn get_test_confused() {
    let mut tree = AVLTree::new();
    tree.insert(4, 4);
    tree.insert(1, 1);
    tree.insert(3, 3);
    tree.insert(2, 2);
    tree.insert(5, 5);
    assert_eq!(tree.get(&4), Some(&4));
}

#[test]
fn min_test() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.min(), Some((&1, &1)));
}

#[test]
fn min_test_confused() {
    let mut tree = AVLTree::new();
    tree.insert(4, 4);
    tree.insert(1, 1);
    tree.insert(3, 3);
    tree.insert(2, 2);
    tree.insert(5, 5);
    assert_eq!(tree.min(), Some((&1, &1)));
}

#[test]
fn min_test_after_removed() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    tree.remove(&1);
    assert_eq!(tree.min(), Some((&2, &2)));
}

#[test]
fn min_test_empty() {
    let tree: AVLTree<i32, i32> = AVLTree::new();
    assert_eq!(tree.min(), None);
}

#[test]
fn max_test() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.max(), Some((&5, &5)));
}

#[test]
fn max_test_confused() {
    let mut tree = AVLTree::new();
    tree.insert(4, 4);
    tree.insert(1, 1);
    tree.insert(3, 3);
    tree.insert(2, 2);
    tree.insert(5, 5);
    assert_eq!(tree.max(), Some((&5, &5)));
}

#[test]
fn max_test_after_removed() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    tree.remove(&5);
    assert_eq!(tree.max(), Some((&4, &4)));
}

#[test]
fn max_test_empty() {
    let tree: AVLTree<i32, i32> = AVLTree::new();
    assert_eq!(tree.max(), None);
}

#[test]
fn size_test() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.size(), 5);
}

#[test]
fn size_test_after_removed() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.remove(&1), Some(1));
    assert_eq!(tree.size(), 4);
}

#[test]
fn size_test_all_removed() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    tree.remove(&1);
    tree.remove(&2);
    tree.remove(&3);
    tree.remove(&4);
    tree.remove(&5);
    assert_eq!(tree.size(), 0);
}

#[test]
fn size_test_empty() {
    let tree: AVLTree<i32, i32> = AVLTree::new();
    assert_eq!(tree.size(), 0);
}

#[test]
fn find_by_order_test() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.find_by_order(0), Some((&1, &1)));
    assert_eq!(tree.find_by_order(1), Some((&2, &2)));
    assert_eq!(tree.find_by_order(2), Some((&3, &3)));
    assert_eq!(tree.find_by_order(3), Some((&4, &4)));
    assert_eq!(tree.find_by_order(4), Some((&5, &5)));
    assert_eq!(tree.find_by_order(5), None);
}

#[test]
fn find_by_order_test_confused() {
    let mut tree = AVLTree::new();
    tree.insert(4, 4);
    tree.insert(1, 1);
    tree.insert(3, 3);
    tree.insert(2, 2);
    tree.insert(5, 5);
    assert_eq!(tree.find_by_order(0), Some((&1, &1)));
    assert_eq!(tree.find_by_order(1), Some((&2, &2)));
    assert_eq!(tree.find_by_order(2), Some((&3, &3)));
    assert_eq!(tree.find_by_order(3), Some((&4, &4)));
    assert_eq!(tree.find_by_order(4), Some((&5, &5)));
    assert_eq!(tree.find_by_order(5), None);
}

#[test]
fn find_by_order_test_after_removed() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.remove(&1), Some(1));
    assert_eq!(tree.find_by_order(0), Some((&2, &2)));
    assert_eq!(tree.find_by_order(1), Some((&3, &3)));
    assert_eq!(tree.find_by_order(2), Some((&4, &4)));
    assert_eq!(tree.find_by_order(3), Some((&5, &5)));
    assert_eq!(tree.find_by_order(4), None);
}

#[test]
fn order_of_key_test() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(4, 3);
    tree.insert(8, 4);
    tree.insert(16, 5);
    assert_eq!(tree.order_of_key(&1), 0);
    assert_eq!(tree.order_of_key(&2), 1);
    assert_eq!(tree.order_of_key(&3), 2);
    assert_eq!(tree.order_of_key(&4), 2);
    assert_eq!(tree.order_of_key(&5), 3);
    assert_eq!(tree.order_of_key(&16), 4);
    assert_eq!(tree.order_of_key(&17), 5);
}

#[test]
fn order_of_key_test_after_removed() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(4, 3);
    tree.insert(8, 4);
    tree.insert(16, 5);
    assert_eq!(tree.remove(&1), Some(1));
    assert_eq!(tree.order_of_key(&2), 0);
    assert_eq!(tree.order_of_key(&3), 1);
    assert_eq!(tree.order_of_key(&4), 1);
    assert_eq!(tree.order_of_key(&5), 2);
    assert_eq!(tree.order_of_key(&16), 3);
    assert_eq!(tree.order_of_key(&17), 4);
}

#[test]
fn order_of_key_test_all_removed() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(4, 3);
    tree.insert(8, 4);
    tree.insert(16, 5);
    tree.remove(&1);
    tree.remove(&2);
    tree.remove(&4);
    tree.remove(&8);
    tree.remove(&16);
    assert_eq!(tree.order_of_key(&1), 0);
    assert_eq!(tree.order_of_key(&2), 0);
    assert_eq!(tree.order_of_key(&3), 0);
    assert_eq!(tree.order_of_key(&4), 0);
    assert_eq!(tree.order_of_key(&5), 0);
    assert_eq!(tree.order_of_key(&16), 0);
    assert_eq!(tree.order_of_key(&17), 0);
}
