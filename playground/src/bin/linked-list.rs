
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
pub struct List<T> {
    pub head: Link<T>,
}

#[derive(Debug, Clone)]
pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    /// push a new node to the list
    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(node);
    }

    /// return the top element
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    /// peek at the top element
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    /// simple iterator
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref()
        }
    }

}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut boxed_node) = link {
            link = boxed_node.next.take();
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub elem: T,
    pub next: Link<T>,
}

fn main() {
    let mut list = List::new();
    list.push(String::from("one"));
    list.push(String::from("two"));
    list.push(String::from("three"));

    for item in list.into_iter() {
        println!("{:?}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let mut list: List<u16> = List::new();
        assert!(list.head.is_none());
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn push_pop() {
        let mut list: List<u16> = List::new();
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        list.push(2);
        assert_eq!(list.peek().unwrap(), &2);
        list.push(3);
        assert_eq!(list.peek().unwrap(), &3);

        println!("{:?}", list);

        let elem = list.pop().unwrap();
        println!("{}", elem);
        assert_eq!(elem, 3);

        let elem = list.pop().unwrap();
        println!("{}", elem);
        assert_eq!(elem, 2);

        let elem = list.pop().unwrap();
        println!("{}", elem);
        assert_eq!(elem, 1);

        assert_eq!(list.pop(), None);
    }
}
