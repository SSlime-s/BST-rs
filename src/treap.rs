use rand::random;

use crate::node::NodeTrait;

struct Node<K, V> {
    key: K,
    value: V,
    left: NodePtr<K, V>,
    right: NodePtr<K, V>,
    size: usize,
    priority: f64,
}
type NodePtrInner<K, V> = Option<Box<Node<K, V>>>;
struct NodePtr<K, V>(NodePtrInner<K, V>);

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
        self.left = NodePtr(node);
    }

    fn set_right(&mut self, node: Option<Box<Self>>) {
        self.right = NodePtr(node);
    }
}
impl<K: Ord, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: NodePtr(None),
            right: NodePtr(None),
            size: 1,
            priority: random::<f64>(),
        }
    }

    fn merge(&mut self, mut other: Self) {
        if self.priority > other.priority {
            self.size += other.size;
            if let Some(mut right) = self.take_right() {
                right.merge(other);
                self.set_right(Some(right));
            } else {
                self.set_right(Some(Box::new(other)));
            }
        } else {
            std::mem::swap(self, &mut other);
            self.size += other.size;
            if let Some(mut left) = self.take_left() {
                left.merge(other);
                self.set_left(Some(left));
            } else {
                self.set_left(Some(Box::new(other)));
            }
        }
    }

    /**
     * (-∞, k) [k, +∞) に分割
     */
    fn split(mut self, key: &K) -> (Option<Self>, Option<Self>) {
        match self.key.cmp(key) {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                let right = match self.take_right() {
                    Some(right) => right,
                    None => return (Some(self), None),
                };
                let (less, greater) = right.split(key);
                self.set_right(less.map(Box::new));
                (Some(self), greater)
            }
            std::cmp::Ordering::Less => {
                let left = match self.take_left() {
                    Some(left) => left,
                    None => return (None, Some(self)),
                };
                let (less, greater) = left.split(key);
                self.set_left(greater.map(Box::new));
                (less, Some(self))
            }
        }
    }
}
impl<K: Ord, V> NodePtr<K, V> {
    fn new(key: K, value: V) -> Self {
        NodePtr(Some(Box::new(Node::new(key, value))))
    }
}

