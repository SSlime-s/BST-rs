extern crate binary_search_tree;
use binary_search_tree::{avl_tree::AVLTree, tree_trait::BinarySearchTree};

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
fn search_test() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.search(&4), Some(&4));
}

#[test]
fn search_test_not_exists() {
    let mut tree = AVLTree::new();
    tree.insert(1, 1);
    tree.insert(2, 2);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    assert_eq!(tree.search(&6), None);
}

#[test]
fn search_test_confused() {
    let mut tree = AVLTree::new();
    tree.insert(4, 4);
    tree.insert(1, 1);
    tree.insert(3, 3);
    tree.insert(2, 2);
    tree.insert(5, 5);
    assert_eq!(tree.search(&4), Some(&4));
}
