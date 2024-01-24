use std::{cmp::Ordering, fs::read_to_string, collections::HashMap};

pub fn solution2() -> i32 {
    let input: Vec<String> = read_to_string("./inputs/input7.txt").unwrap().lines().map(String::from).collect();
    let mut tree: BTree<Hand> = BTree::new();
    for line in input {
        tree.insert(Hand::from(line))
    }
    let mut score = 0;
    for (index, hand) in tree.into_iter().rev().enumerate() {
        let i :i32 = index.try_into().unwrap();
        score += (i+1) * hand.bet
    }
    return score;
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

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bet: i32,
}

impl Hand {
    fn from(s: String) -> Self {
        let mut iter = s.split_whitespace();
        let cards: Vec<Card> = iter.next().unwrap().to_string().chars().map(|c| Card::from(&c)).collect();
        let bet = iter.next().unwrap().parse::<i32>().unwrap();
        let hand_type = Self::get_type(&cards);
        Hand {cards, hand_type, bet}
    }
    fn get_type(cards: &Vec<Card>) -> HandType {
        let mut hashmap = HashMap::<Card,i32>::new();
        for c in cards {
            if let Some(val) = hashmap.get(c) {
                hashmap.insert(*c, val + 1);
            } else {
                hashmap.insert(*c, 1);
            }
        }
        let num_jacks: i32 = *hashmap.get(&Card::Jack).unwrap_or(&0);
        hashmap.remove(&Card::Jack);
        let mut hand_type = HandType::HighCard;
        if num_jacks == 5 {
            hand_type = HandType::Quints;
        }
        for (_,val) in hashmap {
            if val == 2 {
                hand_type = match hand_type {
                    HandType::Triple => HandType::FullHouse,
                    HandType::Pair => HandType::TwoPair,
                    HandType::HighCard => HandType::Pair,
                    _ => hand_type
                };
            } else if val == 3 {
                hand_type = match hand_type {
                    HandType::Pair => HandType::FullHouse,
                    HandType::HighCard => HandType::Triple,
                    _ => hand_type
                };
            } else if val == 4 {
                hand_type = HandType::Quads
            } else if val == 5 {
                hand_type = HandType::Quints
            }
        }
        
        if num_jacks == 1 {
            hand_type = match hand_type {
                HandType::Quads => HandType::Quints,
                HandType::Triple => HandType::Quads,
                HandType::TwoPair => HandType::FullHouse,
                HandType::Pair => HandType::Triple,
                HandType::HighCard => HandType::Pair,
                _ => hand_type
            };
        } else if num_jacks == 2 {
            hand_type = match hand_type {
                HandType::Quads => HandType::Quints,
                HandType::Triple => HandType::Quints,
                HandType::Pair => HandType::Quads,
                HandType::HighCard => HandType::Triple,
                _ => hand_type
            };
            
        } else if num_jacks == 3 {
            hand_type = match hand_type {
                HandType::Pair => HandType::Quints,
                HandType::HighCard => HandType::Quads,
                _ => hand_type
            };
            
        } else if num_jacks == 4 {
            hand_type = HandType::Quints
        }
        
        return hand_type
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let current = self.hand_type.cmp(&other.hand_type);
        if current == Ordering::Equal {
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                let cmp_card = self_card.cmp(other_card);
                if cmp_card != Ordering::Equal {
                    return cmp_card;
                }
            }
        }
        return current
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cards.cmp(&other.cards))
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

#[derive(Debug,PartialEq, PartialOrd,Ord,Eq)]
enum HandType {
    Quints,
    Quads,
    FullHouse,
    Triple,
    TwoPair,
    Pair,
    HighCard
}

#[derive(Debug,PartialEq, PartialOrd,Ord,Eq,Hash,Copy,Clone)]
enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Jack,
}

impl Card {
    fn from(c: &char) -> Card {
        return match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            _ => Card::Two,
        }
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

impl<'a,T:Ord> DoubleEndedIterator for RefNodeIterator<'a,T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let val = self.list.first().map(|node| &node.val);
        if val != None {
            self.list.remove(0);
        }
        return val
    }
}