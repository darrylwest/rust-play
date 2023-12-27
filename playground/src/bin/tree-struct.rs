#![allow(unused)]
#![allow(unused_variables)]

#[derive(Debug, Clone)]
pub struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    pub value: T,
}

impl<T: Ord + Clone> Node<T> {
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

    pub fn find(&self, value: T) -> Option<&Node<T>> {
        if value == self.value {
            Some(self)
        } else if value < self.value {
            self.left.as_ref().and_then(|node| node.find(value))
        } else {
            self.right.as_ref().and_then(|node| node.find(value))
        }
    }
    
    fn walk(&self) -> Vec<T> {
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
    let root_num = 5;
    let mut nums = vec![7, 3, 6, 1, 4];
    let mut root = Node { left: None, right: None, value: root_num };
    
    for n in nums.clone() {
        root.insert(n)
    }
    
    nums.push(5);
    
    // println!("{:?}", root);
    
    for n in nums.into_iter() {
        match root.find(n) {
            Some(v) => println!("found: {:?}", v.value),
            None => println!("{} not found", n),
        }
    }
    
    let list = root.walk();
    println!("{:?}", list);
    
}

