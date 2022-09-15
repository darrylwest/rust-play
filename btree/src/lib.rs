
#[derive(Debug)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

impl<T: Ord> BinaryTree<T> {
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

impl<T: Clone> BinaryTree<T> {
    pub fn walk(&self) -> Vec<T> {
        match *self {
            BinaryTree::Empty => vec![],
            BinaryTree::NonEmpty(ref boxed) => {
                let mut result = boxed.left.walk();
                result.push(boxed.element.clone());
                result.extend(boxed.right.walk());
                result
            },
        }
    }
}

#[derive(Debug)]
pub struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        let list = vec![4, 6, 9, 7, 3, 2, 5];
        // println!("original: {:?}", &list);

        let mut root: BinaryTree<u8> = BinaryTree::Empty;

        for x in &list {
            root.add(*x);
        }

        let sorted = root.walk();
        assert_eq!(sorted, vec![2, 3, 4, 5, 6, 7, 9]);
        assert_eq!(sorted.len(), list.len());
    }

    #[test]
    fn test_string() {
        let list = vec!["fee", "fi", "fo", "fum"];
        let mut root = BinaryTree::Empty;

        for s in &list {
            root.add(*s);
        }

        let sorted = root.walk();
        assert_eq!(sorted, list);
        assert_eq!(sorted.len(), list.len());
    }
}
