
use std::iter::repeat_with;

#[derive(Debug, Clone)]
pub struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: T,
}

impl<T: Ord + Clone> Node<T> {
    /// create a new node
    pub fn create(value: T) -> Node<T> {
        Node { left: None, right: None, value }
    }

    /// read the vector values and create/return a balanced tree
    pub fn from(values: Vec<T>) -> Node<T> {
        let mut list = values.clone();
        list.sort();

        let idx = list.len() / 2;
        let value = list.remove(idx);
        let mut root = Node::create(value);
        while !list.is_empty() {
            let idx = list.len() / 2;
            root.insert(list.remove(idx));
        }

        root
    }

    /// return this node's value
    pub fn value(&self) -> T {
        self.value.clone()
    }

    /// balance the tree and return a new root node
    pub fn balance(&self) -> Node<T> {
        Node::from(self.walk())
    }

    /// insert the value
    pub fn insert(&mut self, value: T) {
        if value < self.value {
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(Node {
                    left: None,
                    right: None,
                    value,
                }));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(Node {
                    left: None,
                    right: None,
                    value,
                }));
            }
        }
    }

    /// find the value (first occurence) and return, else return None
    pub fn find(&self, value: T) -> Option<&Node<T>> {
        if value == self.value {
            Some(self)
        } else if value < self.value {
            self.left.as_ref().and_then(|node| node.find(value))
        } else {
            self.right.as_ref().and_then(|node| node.find(value))
        }
    }
    
    /// walk all nodes in order and return the sorted vector
    pub fn walk(&self) -> Vec<T> {
        let mut values = Vec::new();
        self.walk_recursive(&mut values);
        values
    }

    fn walk_recursive(&self, values: &mut Vec<T>) {
        if let Some(left) = &self.left {
            left.walk_recursive(values);
        }
        values.push(self.value.clone());
        if let Some(right) = &self.right {
            right.walk_recursive(values);
        }
    }
}

fn main() {
    let nums: Vec<u64> = repeat_with(|| fastrand::u64(..)).take(16).collect();
    let root = Node::from(nums.clone());
    
    for n in nums.into_iter() {
        match root.find(n) {
            Some(v) => println!("found: {:?}", v.value),
            None => println!("{} not found", n),
        }
    }
    
    let list = root.walk();
    println!("{:?}", list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_str_tree() {
        let words = vec!["top", "my", "list", "of", "small", "words", "for", "tree"];
        let count = words.len();
        let root = Node::from(words);

        let list = root.walk();
        println!("{:?}", list);
        assert_eq!(count, list.len());

        match root.find("small") {
            Some(v) => {
                println!("{}", v.value());
                assert!(true);
            },
            None => assert!(false, "word not found"),
        }

        // walk a few nodes
        println!("{}", root.value());
        let mut node = root.left;
        while node.is_some() {
            let n = node.unwrap();
            println!("{}, {:?}", n.value(), n.right);
            node = n.left;
        }

        let mut node = root.right;
        while node.is_some() {
            let n = node.unwrap();
            println!("{}, {:?}", n.value(), n.left);
            node = n.right;
        }

    }

    #[test]
    fn build_i32_tree() {
        let mut nums = vec![13, 7, 3, 6, 1, 4];
        let mut root = Node::create(5);

        for n in nums.clone() {
            root.insert(n);
        }

        nums.push(root.value());

        let list = root.walk();
        println!("{:?}", list);
        assert_eq!(list.len(), nums.len());

        match root.find(3) {
            Some(v) => {
                println!("{}", v.value());
                assert!(true);
            },
            None => assert!(false, "number not found"),
        }

        let balanced = root.balance();
        println!("balanced: {:?}", balanced);
    }

    #[test]
    fn from_vector() {
        let mut list = vec![13, 7, 3, 6, 1, 4, 17, 10, 2, 18, 22];
        let root = Node::from(list.clone());

        list.sort();
        let vlist = root.walk();

        println!("{:?}", list);
        println!("{:?}", vlist);

        assert_eq!(list, vlist);
    }

    #[test]
    fn big_numbers() {
        let v: Vec<u64> = repeat_with(|| fastrand::u64(..)).take(25).collect();
        println!("{:?}", v);

        let root = Node::from(v);
        println!("{:?}", root);
    }
}