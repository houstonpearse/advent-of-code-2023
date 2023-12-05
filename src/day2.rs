use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

const MAX_RED:i32 = 12;
const MAX_GREEN:i32 = 13;
const MAX_BLUE:i32 = 14;

pub fn solution1() -> i32 {
    let lines_result: Result<io::Lines<io::BufReader<File>>, io::Error> = read_lines("./inputs/input2.txt");
    let Ok(lines) = lines_result else {   
        println!("{}",lines_result.err().unwrap());
        panic!("Error Reading File");
    };
    let mut sum: i32 = 0;
    for line in lines {
        if let Ok(game_string) = line {
            let game: Game = Game::parse_game(&game_string);
            if game.is_valid() {
                sum+=game.n;
            }
        }
    };
    return sum
}

pub fn solution2() -> i32 {
    let lines_result: Result<io::Lines<io::BufReader<File>>, io::Error> = read_lines("./inputs/input2.txt");
    let Ok(lines) = lines_result else {   
        println!("{}",lines_result.err().unwrap());
        panic!("Error Reading File");
    };
    let mut sum: i32 = 0;
    for line in lines {
        if let Ok(game_string) = line {
            let game: Game = Game::parse_game(&game_string);
            sum+=game.minimum_set().power();
        }
    };
    return sum
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Game {
    n: i32,
    hands: Vec<Hand>,
}

#[derive(Debug)]
struct Hand {
    r: i32,
    g: i32,
    b: i32,
}

impl Game {
    fn parse_game(game:&String) -> Game {
        let game_regex: Regex = Regex::new("(?<game>[0-9]+)").unwrap();
        let hand_regex: Regex = Regex::new("(?<num>[0-9]+) (?<colour>red|green|blue)").unwrap();
        let game_split: Vec<&str> = game.split(":").collect();
        let first_half: &str = game_split[0];
        let second_half: &str = game_split[1];

        let caps: regex::Captures<'_> = game_regex.captures(first_half).unwrap();
        let game_num: i32 = caps["game"].parse::<i32>().unwrap();
        let mut hands_vec: Vec<Hand> = Vec::new();
        let hands:Vec<&str> = second_half.split(";").collect();
        for hand in hands {
            let mut red: i32 = 0;
            let mut green: i32 = 0;
            let mut blue: i32 = 0;
            for colour_group in hand_regex.captures_iter(hand) {
                if &colour_group["colour"] == "red" {
                    red = colour_group["num"].parse::<i32>().unwrap();
                } else if &colour_group["colour"] == "green" {
                    green = colour_group["num"].parse::<i32>().unwrap();
                } else {
                    blue = colour_group["num"].parse::<i32>().unwrap();
                }
            }
            hands_vec.push(Hand {r: red, g: green, b: blue})
        }
        return Game {n: game_num, hands: hands_vec};
    }

    fn is_valid(&self) -> bool {
        !self.hands.iter().any(|x: &Hand| !x.is_valid())
    }

    fn minimum_set(&self) -> Hand {
        let mut r: i32 = 0;
        let mut g: i32 = 0;
        let mut b: i32 = 0;
        for hand in self.hands.iter() {
            if hand.r > r {
                r = hand.r;
            }
            if hand.g > g {
                g = hand.g
            }
            if hand.b > b {
                b = hand.b
            }
        }
        return Hand {r,g,b}
    }
}

impl Hand {
    fn is_valid(&self) -> bool {
        return self.r <= MAX_RED && self.g <=MAX_GREEN && self.b <= MAX_BLUE;
    }
    fn power(&self) -> i32 {
        return self.r * self.g * self.b;
    }
}
