use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn solution1() -> i32 {
    let lines_result: Result<io::Lines<io::BufReader<File>>, io::Error> = read_lines("./inputs/input4.txt");
    let Ok(lines) = lines_result else {   
        println!("{}",lines_result.err().unwrap());
        panic!("Error Reading File");
    };
    let mut sum: i32 = 0;
    for line in lines {
        if let Ok(card_string) = line {
            let card: Card = Card::from(card_string);
            let mut score: i32 = 0;
            for num in card.card_nums {
                if card.win_nums.contains(&num) {
                    if score == 0 {
                        score = 1;
                    } else {
                        score *= 2;
                    }
                }
            }
            sum+=score;
        }
    };
    return sum
}

pub fn solution2() -> i32 {
    let lines_result: Result<io::Lines<io::BufReader<File>>, io::Error> = read_lines("./inputs/input4.txt");
    let Ok(lines) = lines_result else {   
        println!("{}",lines_result.err().unwrap());
        panic!("Error Reading File");
    };
    let mut cards: Vec<Card> = Vec::new();
    for line in lines {
        if let Ok(card_string) = line {
            cards.push(Card::from(card_string));
        }
    };
    for index in 0..cards.len() {
        let matches = cards[index].get_matches();
        for i in index+1..index+1+matches as usize {
            if i < cards.len() {
                cards[i].copies += cards[index].copies;
            }
        }
    }
    let mut sum: i32 = 0;
    for card in cards {
        sum+=card.copies;
    }

    return sum
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Card {
    win_nums: Vec<i32>,
    card_nums: Vec<i32>,
    copies: i32,
}

impl Card {
    fn from(card: String) -> Card {
        let card_split: Vec<&str> = card.split(":").collect();
        let numbers: Vec<&str> = card_split[1].split("|").collect();
        let win_nums: Vec<i32> = numbers[0]
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let card_nums :Vec<i32>= numbers[1]
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        return Card {win_nums, card_nums, copies: 1};
    }

    fn get_matches(&self) -> i32 {
        let mut matches: i32 = 0;
        for num in self.card_nums.iter() {
            if self.win_nums.contains(num) {
                matches+=1;
            }
        }
        return matches;
    }

}