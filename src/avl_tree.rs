use std::ops::{Deref, DerefMut};

use crate::tree_trait::BinarySearchTree;

#[derive(Clone, Copy, PartialEq)]
enum ThreeWay {
    Left,
    Right,
    Equal,
}

type NodePtrInner<K, V> = Option<Box<Node<K, V>>>;
struct NodePtr<K, V>(NodePtrInner<K, V>);
struct Node<K, V> {
    key: K,
    value: V,
    left: NodePtr<K, V>,
    right: NodePtr<K, V>,
    size: usize,
    state: ThreeWay,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: NodePtr(None),
            right: NodePtr(None),
            size: 1,
            state: ThreeWay::Equal,
        }
    }
}

impl<K, V> Deref for NodePtr<K, V> {
    type Target = NodePtrInner<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K, V> DerefMut for NodePtr<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<K, V> From<NodePtrInner<K, V>> for NodePtr<K, V> {
    fn from(node: NodePtrInner<K, V>) -> Self {
        NodePtr(node)
    }
}
impl<K, V> NodePtr<K, V> {
    fn new(key: K, value: V) -> Self {
        NodePtr(Some(Box::new(Node::new(key, value))))
    }

    fn size(&self) -> usize {
        self.0.as_ref().map_or(0, |node| node.size)
    }

    fn rotate_right(&mut self) {
        let mut node = match self.take() {
            Some(node) => node,
            None => return,
        };

        let mut left = match node.left.take() {
            Some(left) => left,
            None => return,
        };

        node.size = node.size - left.size + left.right.size();
        node.left = left.right.take().into();

        left.size = left.size - left.right.size() + node.size;
        left.right = Some(node).into();

        *self = Some(left).into();
    }

    fn rotate_left(&mut self) {
        let mut node = match self.take() {
            Some(node) => node,
            None => return,
        };

        let mut right = match node.right.take() {
            Some(right) => right,
            None => return,
        };

        node.size = node.size - right.size + right.left.size();
        node.right = right.left.take().into();

        right.size = right.size - right.left.size() + node.size;
        right.left = Some(node).into();

        *self = Some(right).into();
    }
}

impl<K: Ord, V> NodePtr<K, V> {
    fn insert_rec(&mut self, key: K, value: V) -> (bool, bool) {
        let mut node = match self.take() {
            Some(node) => node,
            None => {
                *self = Some(Box::new(Node::new(key, value))).into();
                return (true, true);
            }
        };

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => {
                let (left_inserted, left_changed) = node.left.insert_rec(key, value);
                if !left_inserted {
                    *self = Some(node).into();
                    return (false, false);
                }
                node.size += 1;
                if !left_changed {
                    *self = Some(node).into();
                    return (true, false);
                }

                match node.state {
                    ThreeWay::Equal => {
                        node.state = ThreeWay::Left;
                        *self = Some(node).into();
                        (true, true)
                    }
                    ThreeWay::Left => {
                        match node.left.as_ref().unwrap().state {
                            ThreeWay::Left => {
                                let mut wrapped_node: NodePtr<_, _> = Some(node).into();
                                wrapped_node.rotate_right();
                                let mut node = wrapped_node.take().unwrap();
                                node.state = ThreeWay::Equal;
                                if let Some(mut right) = node.right.as_mut() {
                                    right.state = ThreeWay::Equal;
                                }
                                *self = Some(node).into();
                            }
                            ThreeWay::Right => {
                                let state =
                                    node.left.as_ref().unwrap().right.as_ref().unwrap().state;
                                node.left.rotate_left();
                                let mut wrapped_node: NodePtr<_, _> = Some(node).into();
                                wrapped_node.rotate_right();
                                let mut node = wrapped_node.take().unwrap();
                                node.state = ThreeWay::Equal;
                                if let Some(mut right) = node.right.as_mut() {
                                    right.state = match state {
                                        ThreeWay::Left => ThreeWay::Right,
                                        ThreeWay::Right => ThreeWay::Equal,
                                        ThreeWay::Equal => unreachable!(),
                                    };
                                }
                                if let Some(mut left) = node.left.as_mut() {
                                    left.state = match state {
                                        ThreeWay::Left => ThreeWay::Equal,
                                        ThreeWay::Right => ThreeWay::Left,
                                        ThreeWay::Equal => unreachable!(),
                                    };
                                }
                                *self = Some(node).into();
                            }
                            ThreeWay::Equal => {
                                unreachable!()
                            }
                        }
                        (true, false)
                    }
                    ThreeWay::Right => {
                        node.state = ThreeWay::Equal;
                        *self = Some(node).into();
                        (true, false)
                    }
                }
            }
            std::cmp::Ordering::Equal => {
                node.value = value;
                *self = Some(node).into();
                (false, false)
            }
            std::cmp::Ordering::Greater => {
                let (right_inserted, right_changed) = node.right.insert_rec(key, value);
                if !right_inserted {
                    *self = Some(node).into();
                    return (false, false);
                }
                node.size += 1;
                if !right_changed {
                    *self = Some(node).into();
                    return (true, false);
                }

                match node.state {
                    ThreeWay::Equal => {
                        node.state = ThreeWay::Right;
                        *self = Some(node).into();
                        (true, true)
                    }
                    ThreeWay::Left => {
                        node.state = ThreeWay::Equal;
                        *self = Some(node).into();
                        (true, false)
                    }
                    ThreeWay::Right => {
                        match node.right.as_ref().unwrap().state {
                            ThreeWay::Right => {
                                let mut wrapped_node: NodePtr<_, _> = Some(node).into();
                                wrapped_node.rotate_left();
                                let mut node = wrapped_node.take().unwrap();
                                node.state = ThreeWay::Equal;
                                if let Some(mut left) = node.left.as_mut() {
                                    left.state = ThreeWay::Equal;
                                }
                                *self = Some(node).into();
                            }
                            ThreeWay::Left => {
                                let state =
                                    node.right.as_ref().unwrap().left.as_ref().unwrap().state;
                                node.right.rotate_right();
                                let mut wrapped_node: NodePtr<_, _> = Some(node).into();
                                wrapped_node.rotate_left();
                                let mut node = wrapped_node.take().unwrap();
                                node.state = ThreeWay::Equal;
                                if let Some(mut left) = node.left.as_mut() {
                                    left.state = match state {
                                        ThreeWay::Left => ThreeWay::Equal,
                                        ThreeWay::Right => ThreeWay::Right,
                                        ThreeWay::Equal => unreachable!(),
                                    };
                                }
                                if let Some(mut right) = node.right.as_mut() {
                                    right.state = match state {
                                        ThreeWay::Left => ThreeWay::Left,
                                        ThreeWay::Right => ThreeWay::Equal,
                                        ThreeWay::Equal => unreachable!(),
                                    };
                                }
                                *self = Some(node).into();
                            }
                            ThreeWay::Equal => {
                                unreachable!()
                            }
                        }
                        (true, false)
                    }
                }
            }
        }
    }
}

impl<K, V> BinarySearchTree<K, V> for NodePtr<K, V>
where
    K: Ord,
{
    fn insert(&mut self, key: K, value: V) -> bool {
        self.insert_rec(key, value).0
    }

    fn remove(&mut self, key: K) -> Option<V> {
        todo!()
    }

    fn search(&self, key: K) -> Option<V> {
        todo!()
    }

    fn min(&self) -> Option<(&K, &V)> {
        todo!()
    }

    fn max(&self) -> Option<(&K, &V)> {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn find_by_order(&self, order: usize) -> Option<(&K, &V)> {
        todo!()
    }

    fn order_of_key(&self, key: K) -> Option<usize> {
        todo!()
    }
}
