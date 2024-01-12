use std::cmp::Ordering;

pub fn solution1() -> i32 {
    let mut tree: BTree<Hand> = Option::None;
    return 0;
}

pub fn solution2() -> i64 {
    return 0
}

type BTree<T: Ord> = Option<Box<Node<T>>>;

struct Node<T: Ord> {
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
                match &self.left {
                    None => {
                        self.left = Some(Box::new(Node::<T>::new(t)))
                    }
                    Some(node) => node.insert(t)
                }
            },
            Ordering::Greater => {

            },
        };
    }
}

#[derive(PartialEq,Eq,PartialOrd)]
struct Hand {
    cards: String,
    bet: i32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.cards.cmp(&other.cards)
    }
}