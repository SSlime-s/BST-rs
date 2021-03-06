// verified by https://judge.yosupo.jp/submission/70311

use std::iter::FromIterator;

use crate::node::NodeTrait;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ThreeWay {
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
impl<K, V> NodeTrait for Node<K, V> {
    fn size(&self) -> usize {
        self.size
    }

    fn size_mut(&mut self) -> &mut usize {
        &mut self.size
    }

    fn left(&self) -> Option<&Self> {
        self.left.0.as_deref()
    }

    fn right(&self) -> Option<&Self> {
        self.right.0.as_deref()
    }

    fn take_left(&mut self) -> Option<Box<Self>> {
        self.left.0.take()
    }

    fn take_right(&mut self) -> Option<Box<Self>> {
        self.right.0.take()
    }

    fn set_left(&mut self, node: Option<Box<Self>>) {
        self.left.0 = node;
    }

    fn set_right(&mut self, node: Option<Box<Self>>) {
        self.right.0 = node;
    }
}

impl<'a, K, V> IntoIterator for &'a NodePtr<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = Vec::new();
        if let Some(node) = &self.0 {
            vec.extend(node.left.into_iter());
            vec.push((&node.key, &node.value));
            vec.extend(node.right.into_iter());
        }
        vec.into_iter()
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

    fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    fn keys(&self) -> Vec<&K> {
        let mut vec = Vec::new();
        if let Some(node) = &self.0 {
            vec.extend(node.left.keys());
            vec.push(&node.key);
            vec.extend(node.right.keys());
        }
        vec
    }

    fn values(&self) -> Vec<&V> {
        let mut vec = Vec::new();
        if let Some(node) = &self.0 {
            vec.extend(node.left.values());
            vec.push(&node.value);
            vec.extend(node.right.values());
        }
        vec
    }
}

impl<K: Ord, V> NodePtr<K, V> {
    fn insert_rec(&mut self, key: K, value: V) -> (bool, bool) {
        let mut node = match self.0.take() {
            Some(node) => node,
            None => {
                *self = NodePtr::new(key, value);
                return (true, true);
            }
        };

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => {
                let (left_inserted, left_increased) = node.left.insert_rec(key, value);
                if !left_inserted {
                    *self = Some(node).into();
                    return (false, false);
                }
                node.size += 1;
                if !left_increased {
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
                        match node.left.0.as_ref().unwrap().state {
                            ThreeWay::Left => {
                                node.rotate_right();
                                node.state = ThreeWay::Equal;
                                if let Some(mut right) = node.right.0.as_mut() {
                                    right.state = ThreeWay::Equal;
                                }
                                *self = Some(node).into();
                            }
                            ThreeWay::Right => {
                                let state = node
                                    .left
                                    .0
                                    .as_ref()
                                    .unwrap()
                                    .right
                                    .0
                                    .as_ref()
                                    .unwrap()
                                    .state;
                                node.left.0.as_mut().unwrap().rotate_left();
                                node.rotate_right();
                                node.state = ThreeWay::Equal;
                                if let Some(mut right) = node.right.0.as_mut() {
                                    right.state = match state {
                                        ThreeWay::Left => ThreeWay::Right,
                                        ThreeWay::Right | ThreeWay::Equal => ThreeWay::Equal,
                                    };
                                }
                                if let Some(mut left) = node.left.0.as_mut() {
                                    left.state = match state {
                                        ThreeWay::Left | ThreeWay::Equal => ThreeWay::Equal,
                                        ThreeWay::Right => ThreeWay::Left,
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
                let (right_inserted, right_increased) = node.right.insert_rec(key, value);
                if !right_inserted {
                    *self = Some(node).into();
                    return (false, false);
                }
                node.size += 1;
                if !right_increased {
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
                        match node.right.0.as_ref().unwrap().state {
                            ThreeWay::Right => {
                                node.rotate_left();
                                node.state = ThreeWay::Equal;
                                if let Some(mut left) = node.left.0.as_mut() {
                                    left.state = ThreeWay::Equal;
                                }
                                *self = Some(node).into();
                            }
                            ThreeWay::Left => {
                                let state = node
                                    .right
                                    .0
                                    .as_ref()
                                    .unwrap()
                                    .left
                                    .0
                                    .as_ref()
                                    .unwrap()
                                    .state;
                                node.right.0.as_mut().unwrap().rotate_right();
                                node.rotate_left();
                                node.state = ThreeWay::Equal;
                                if let Some(mut left) = node.left.0.as_mut() {
                                    left.state = match state {
                                        ThreeWay::Left | ThreeWay::Equal => ThreeWay::Equal,
                                        ThreeWay::Right => ThreeWay::Left,
                                    };
                                }
                                if let Some(mut right) = node.right.0.as_mut() {
                                    right.state = match state {
                                        ThreeWay::Left => ThreeWay::Right,
                                        ThreeWay::Right | ThreeWay::Equal => ThreeWay::Equal,
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

    fn remove_rec(&mut self, key: &K) -> (Option<(K, V)>, bool) {
        let mut node = match self.0.take() {
            Some(node) => node,
            None => return (None, false),
        };

        match key.cmp(&node.key) {
            std::cmp::Ordering::Equal => match (node.left.0.take(), node.right.0.take()) {
                (None, None) => (Some((node.key, node.value)), true),
                (Some(left), None) => {
                    *self = Some(left).into();
                    (Some((node.key, node.value)), true)
                }
                (None, Some(right)) => {
                    *self = Some(right).into();
                    (Some((node.key, node.value)), true)
                }
                (Some(left), Some(right)) => {
                    // ?????????????????????????????????2????????????????????????
                    // ????????????????????????????????? ???????????????????????????????????????????????????
                    let mut wrapped_left: NodePtr<_, _> = Some(left).into();
                    let (value, decreased) = wrapped_left.remove_max_rec();
                    let (mut key, mut value) = value.unwrap();
                    std::mem::swap(&mut node.key, &mut key);
                    std::mem::swap(&mut node.value, &mut value);
                    node.size -= 1;
                    node.left = wrapped_left;
                    node.right = Some(right).into();
                    match node.state {
                        _ if !decreased => {
                            *self = Some(node).into();
                            (Some((key, value)), false)
                        }
                        ThreeWay::Equal => {
                            node.state = ThreeWay::Right;
                            *self = Some(node).into();
                            (Some((key, value)), false)
                        }
                        ThreeWay::Left => {
                            node.state = ThreeWay::Equal;
                            *self = Some(node).into();
                            (Some((key, value)), true)
                        }
                        ThreeWay::Right => {
                            *self = Some(node).into();
                            (Some((key, value)), self.rebalanced_for_left_remove())
                        }
                    }
                }
            },
            std::cmp::Ordering::Greater => {
                let (value, decreased) = node.right.remove_rec(key);
                if value.is_none() {
                    *self = Some(node).into();
                    return (None, false);
                }
                node.size -= 1;
                if !decreased {
                    *self = Some(node).into();
                    return (value, false);
                }
                match node.state {
                    ThreeWay::Equal => {
                        node.state = ThreeWay::Left;
                        *self = Some(node).into();
                        (value, false)
                    }
                    ThreeWay::Right => {
                        node.state = ThreeWay::Equal;
                        *self = Some(node).into();
                        (value, true)
                    }
                    ThreeWay::Left => {
                        *self = Some(node).into();
                        (value, self.rebalanced_for_right_remove())
                    }
                }
            }
            std::cmp::Ordering::Less => {
                let (value, decreased) = node.left.remove_rec(key);
                if value.is_none() {
                    *self = Some(node).into();
                    return (None, false);
                }
                node.size -= 1;
                if !decreased {
                    *self = Some(node).into();
                    return (value, false);
                }
                match node.state {
                    ThreeWay::Equal => {
                        node.state = ThreeWay::Right;
                        *self = Some(node).into();
                        (value, false)
                    }
                    ThreeWay::Left => {
                        node.state = ThreeWay::Equal;
                        *self = Some(node).into();
                        (value, true)
                    }
                    ThreeWay::Right => {
                        *self = Some(node).into();
                        (value, self.rebalanced_for_left_remove())
                    }
                }
            }
        }
    }

    fn remove_max_rec(&mut self) -> (Option<(K, V)>, bool) {
        let mut node = match self.0.take() {
            Some(node) => node,
            None => return (None, false),
        };

        match node.right.0.take() {
            None => {
                let left = node.left.0.take();
                *self = left.into();
                (Some((node.key, node.value)), true)
            }
            Some(right) => {
                let mut wrapped_right: NodePtr<_, _> = Some(right).into();
                let (ret, decreased) = wrapped_right.remove_max_rec();
                node.right = wrapped_right;
                node.size -= 1;
                if !decreased {
                    *self = Some(node).into();
                    return (ret, false);
                }

                match node.state {
                    ThreeWay::Equal => {
                        node.state = ThreeWay::Left;
                        *self = Some(node).into();
                        (ret, false)
                    }
                    ThreeWay::Right => {
                        node.state = ThreeWay::Equal;
                        *self = Some(node).into();
                        (ret, true)
                    }
                    ThreeWay::Left => {
                        *self = Some(node).into();
                        (ret, self.rebalanced_for_right_remove())
                    }
                }
            }
        }
    }

    // ???????????????????????????????????????????????????????????????????????????????????? 2 ??????????????????????????????
    // ????????????????????????????????????????????????????????????????????????
    fn rebalanced_for_left_remove(&mut self) -> bool {
        let mut node = self.0.take().unwrap();
        let right_state = node.right.0.as_ref().unwrap().state;
        match right_state {
            ThreeWay::Equal => {
                node.rotate_left();
                node.state = ThreeWay::Left;
                *self = Some(node).into();
                false
            }
            ThreeWay::Right => {
                node.rotate_left();
                node.state = ThreeWay::Equal;
                if let Some(mut left) = node.left.0.as_mut() {
                    left.state = ThreeWay::Equal;
                }
                *self = Some(node).into();
                true
            }
            ThreeWay::Left => {
                let state = node
                    .right
                    .0
                    .as_ref()
                    .unwrap()
                    .left
                    .0
                    .as_ref()
                    .unwrap()
                    .state;
                node.right.0.as_mut().unwrap().rotate_right();
                node.rotate_left();
                node.state = ThreeWay::Equal;
                if let Some(mut left) = node.left.0.as_mut() {
                    left.state = match state {
                        ThreeWay::Left | ThreeWay::Equal => ThreeWay::Equal,
                        ThreeWay::Right => ThreeWay::Left,
                    };
                }
                if let Some(mut right) = node.right.0.as_mut() {
                    right.state = match state {
                        ThreeWay::Left => ThreeWay::Right,
                        ThreeWay::Right | ThreeWay::Equal => ThreeWay::Equal,
                    };
                }
                *self = Some(node).into();
                true
            }
        }
    }

    // ???????????????????????????????????????????????????????????????????????????????????? 2 ??????????????????????????????
    // ????????????????????????????????????????????????????????????????????????
    fn rebalanced_for_right_remove(&mut self) -> bool {
        let mut node = self.0.take().unwrap();
        let left_state = node.left.0.as_ref().unwrap().state;
        match left_state {
            ThreeWay::Equal => {
                node.rotate_right();
                node.state = ThreeWay::Right;
                *self = Some(node).into();
                false
            }
            ThreeWay::Left => {
                node.rotate_right();
                node.state = ThreeWay::Equal;
                if let Some(mut right) = node.right.0.as_mut() {
                    right.state = ThreeWay::Equal;
                }
                *self = Some(node).into();
                true
            }
            ThreeWay::Right => {
                let state = node
                    .left
                    .0
                    .as_ref()
                    .unwrap()
                    .right
                    .0
                    .as_ref()
                    .unwrap()
                    .state;
                node.left.0.as_mut().unwrap().rotate_left();
                node.rotate_right();
                node.state = ThreeWay::Equal;
                if let Some(mut left) = node.left.0.as_mut() {
                    left.state = match state {
                        ThreeWay::Left | ThreeWay::Equal => ThreeWay::Equal,
                        ThreeWay::Right => ThreeWay::Left,
                    };
                }
                if let Some(mut right) = node.right.0.as_mut() {
                    right.state = match state {
                        ThreeWay::Left => ThreeWay::Right,
                        ThreeWay::Right | ThreeWay::Equal => ThreeWay::Equal,
                    };
                }
                *self = Some(node).into();
                true
            }
        }
    }
}

impl<K, V> NodePtr<K, V>
where
    K: Ord,
{
    fn insert(&mut self, key: K, value: V) -> bool {
        self.insert_rec(key, value).0
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove_rec(key).0.map(|node| node.1)
    }

    fn search(&self, key: &K) -> Option<&V> {
        match self.0.as_ref() {
            None => None,
            Some(node) => match key.cmp(&node.key) {
                std::cmp::Ordering::Less => node.left.search(key),
                std::cmp::Ordering::Greater => node.right.search(key),
                std::cmp::Ordering::Equal => Some(&node.value),
            },
        }
    }

    fn search_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.0.as_mut() {
            None => None,
            Some(node) => match key.cmp(&node.key) {
                std::cmp::Ordering::Less => node.left.search_mut(key),
                std::cmp::Ordering::Greater => node.right.search_mut(key),
                std::cmp::Ordering::Equal => Some(&mut node.value),
            },
        }
    }

    fn min(&self) -> Option<(&K, &V)> {
        match self.0.as_ref() {
            Some(mut node) => {
                while let Some(nxt) = node.left.0.as_ref() {
                    node = nxt;
                }
                Some((&node.key, &node.value))
            }
            None => None,
        }
    }

    fn max(&self) -> Option<(&K, &V)> {
        match self.0.as_ref() {
            Some(mut node) => {
                while let Some(nxt) = node.right.0.as_ref() {
                    node = nxt;
                }
                Some((&node.key, &node.value))
            }
            None => None,
        }
    }

    fn find_by_order(&self, order: usize) -> Option<(&K, &V)> {
        if self.size() <= order {
            return None;
        }

        let mut rest = order;
        let mut node = self.0.as_ref().unwrap();
        loop {
            match node.left.size().cmp(&rest) {
                std::cmp::Ordering::Less => {
                    rest -= node.left.size() + 1;
                    node = node.right.0.as_ref().unwrap();
                }
                std::cmp::Ordering::Greater => {
                    node = node.left.0.as_ref().unwrap();
                }
                std::cmp::Ordering::Equal => {
                    break Some((&node.key, &node.value));
                }
            }
        }
    }

    fn order_of_key(&self, key: &K) -> usize {
        let mut order = 0;
        let mut node = match self.0.as_ref() {
            None => return 0,
            Some(node) => node,
        };
        loop {
            match key.cmp(&node.key) {
                std::cmp::Ordering::Less => {
                    node = match node.left.0.as_ref() {
                        Some(node) => node,
                        None => break order,
                    };
                }
                std::cmp::Ordering::Greater => {
                    order += node.left.size() + 1;
                    node = match node.right.0.as_ref() {
                        Some(node) => node,
                        None => break order,
                    };
                }
                std::cmp::Ordering::Equal => {
                    break order + node.left.size();
                }
            }
        }
    }
}

pub struct AVLTreeMap<K: Ord, V> {
    root: NodePtr<K, V>,
}
impl<K: Ord, V> AVLTreeMap<K, V> {
    pub fn new() -> Self {
        AVLTreeMap { root: None.into() }
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        self.root.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.root.remove(key)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.search(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.root.search_mut(key)
    }

    pub fn min(&self) -> Option<(&K, &V)> {
        self.root.min()
    }

    pub fn max(&self) -> Option<(&K, &V)> {
        self.root.max()
    }

    pub fn size(&self) -> usize {
        self.root.size()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn find_by_order(&self, order: usize) -> Option<(&K, &V)> {
        self.root.find_by_order(order)
    }

    /**
     * key ???????????????????????????????????????
     */
    pub fn order_of_key(&self, key: &K) -> usize {
        self.root.order_of_key(key)
    }

    pub fn keys(&self) -> Vec<&K> {
        self.root.keys()
    }

    pub fn values(&self) -> Vec<&V> {
        self.root.values()
    }
}
impl<'a, K: Ord, V> IntoIterator for &'a AVLTreeMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.root.into_iter()
    }
}
impl<K: Ord, V> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Ord, V> FromIterator<(K, V)> for AVLTreeMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::new();
        for (key, value) in iter {
            tree.insert(key, value);
        }
        tree
    }
}

pub struct AVLTreeSet<K: Ord> {
    root: NodePtr<K, ()>,
}
impl<K: Ord> AVLTreeSet<K> {
    pub fn new() -> Self {
        AVLTreeSet { root: None.into() }
    }

    pub fn insert(&mut self, key: K) -> bool {
        self.root.insert(key, ())
    }

    pub fn remove(&mut self, key: &K) -> bool {
        self.root.remove(key).is_some()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.root.search(key).is_some()
    }

    pub fn min(&self) -> Option<&K> {
        self.root.min().map(|(k, _)| k)
    }

    pub fn max(&self) -> Option<&K> {
        self.root.max().map(|(k, _)| k)
    }

    pub fn size(&self) -> usize {
        self.root.size()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn find_by_order(&self, order: usize) -> Option<&K> {
        self.root.find_by_order(order).map(|(k, _)| k)
    }

    /**
     * key ???????????????????????????????????????
     */
    pub fn order_of_key(&self, key: &K) -> usize {
        self.root.order_of_key(key)
    }

    pub fn keys(&self) -> Vec<&K> {
        self.root.keys()
    }
}
impl<'a, K: Ord> IntoIterator for &'a AVLTreeSet<K> {
    type Item = &'a K;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.root
            .into_iter()
            .map(|(k, _)| k)
            .collect::<Vec<_>>()
            .into_iter()
    }
}
impl<K: Ord> Default for AVLTreeSet<K> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Ord> FromIterator<K> for AVLTreeSet<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut tree = Self::new();
        for key in iter {
            tree.insert(key);
        }
        tree
    }
}
