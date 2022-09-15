
#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
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
    fn walk(&self) -> Vec<T> {
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
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

fn main() {
    let list = vec![4, 6, 9, 7, 3, 2, 5];
    println!("original: {:?}", &list);

    let mut root: BinaryTree<u8> = BinaryTree::Empty;

    for x in list {
        root.add(x);
    }


    let sorted = root.walk();
    println!("sorted: {:?}", &sorted);

}
