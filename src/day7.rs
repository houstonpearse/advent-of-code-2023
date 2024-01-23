use std::{cmp::Ordering, fs::read_to_string};

pub fn solution1() -> i32 {
    let input: Vec<String> = read_to_string("./inputs/input7.txt").unwrap().lines().map(String::from).collect();
    let mut tree: BTree<Hand> = BTree::new();
    for line in input {
        tree.insert(Hand::from(line))
    }
    for hand in &tree {
        println!("{:?}",hand);
    }
    return 0;
}

pub fn solution2() -> i64 {
    return 0
}

#[derive(Debug)]
struct BTree<T>(Option<Box<Node<T>>>);

impl<T: Ord> BTree<T> {
    fn insert(&mut self, t:T) {
        if let BTree(Some(node)) = self {
            node.insert(t);
        } else {
            *self = BTree(Some(Box::new(Node::new(t))))
        } 
    }
    fn new() -> Self {
        return BTree(None);
    }
}

impl<'a, T:Ord> IntoIterator for &'a BTree<T> {
    type Item = &'a T;

    type IntoIter = RefNodeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        if let BTree(Some(node)) = self {
            return node.into_iter();
        } else {
            return RefNodeIterator { list: Vec::new() }

        };
    }
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(t: T) -> Self {
        return Node {
            val: t,
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, t: T) {
        match self.val.cmp(&t) {
            Ordering::Less | Ordering::Equal => {
                match self.left {
                    None => {
                        self.left = Some(Box::new(Node::<T>::new(t)))
                    }
                    Some(ref mut node ) => node.insert(t)
                }
            },
            Ordering::Greater => {
                match self.right {
                    None => {
                        self.right = Some(Box::new(Node::<T>::new(t)))
                    }
                    Some(ref mut node ) => node.insert(t)
                }
            }
        };
    }
}

impl<'a, T:Ord> IntoIterator for &'a Node<T> {
    type Item = &'a T;

    type IntoIter = RefNodeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut new_list: Vec<&Node<T>> = Vec::new();
        if let Some(node) = &self.left {
            new_list.append(node.into_iter().list.as_mut());
        }
        new_list.push(self);
        if let Some(node) = &self.right {
            new_list.append(node.into_iter().list.as_mut());
        }
        RefNodeIterator {list: new_list}
    }
}

#[derive(PartialEq,Eq,PartialOrd,Debug)]
struct Hand {
    cards: String,
    bet: i32,
}

impl Hand {
    fn from(s: String) -> Self {
        let mut iter = s.split_whitespace();
        Hand {cards: iter.next().unwrap().to_string(), bet: iter.next().unwrap().parse::<i32>().unwrap()}
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.cards.cmp(&other.cards)
    }
}

struct RefNodeIterator<'a, T> {
    list: Vec<&'a Node<T>>,
}

impl<'a,T:Ord> Iterator for RefNodeIterator<'a,T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        return self.list.pop().map(|node| &node.val);
    }
}