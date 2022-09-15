
#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
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

    fn traverse(&self) {
        match *self {
            BinaryTree::NonEmpty(ref node) => {
                node.left.traverse(list);
                node.right.traverse(list);
            },
            BinaryTree::Empty => node.element;
        }
    }
}

fn main() {
    let list = vec![6, 7, 3, 2, 5];

    let mut root: BinaryTree<u8> = BinaryTree::Empty;

    for x in list {
        root.add(x);
    }

    // println!("{:#?}", root);

    root.traverse();

}
