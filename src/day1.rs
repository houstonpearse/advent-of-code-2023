use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn solution1()-> i32 {
    let lines_result: Result<io::Lines<io::BufReader<File>>, io::Error> = read_lines("./inputs/input1.txt");
    let Ok(lines) = lines_result else {   
        println!("{}",lines_result.err().unwrap());
        panic!("Error Reading File");
    };
    let mut sum: i32 = 0;
    for line in lines {
        if let Ok(code) = line {
            let (first,last) = first_last_digits(code);
            let mut number: String = first.to_string();
            number.push(last);
            let calibration_value: i32 = number.parse().unwrap();
            sum += calibration_value;
        }
    };
    return sum;
}

fn first_last_digits(code: String) -> (char, char) {
    let digits: Vec<char> = code.chars().filter(|x| x.is_digit(10)).collect();
    return (digits[0],digits[digits.len()-1])
}

pub fn solution2() -> i32 {
    let lines_result: Result<io::Lines<io::BufReader<File>>, io::Error> = read_lines("./inputs/input1.txt");
    let Ok(lines) = lines_result else {   
        println!("{}",lines_result.err().unwrap());
        panic!("Error reading file");
    };
    let mut sum = 0;
    for line in lines {
        if let Ok(code) = line {
            let mut number_strings: Vec<(i32, i32)> = find_number_strings(&code);
            let mut number_digits: Vec<(i32, i32)> = find_number_digits(&code);
            let mut numbers: Vec<(i32, i32)> = Vec::new();
            numbers.append(&mut number_strings);
            numbers.append(&mut number_digits);
            numbers.sort_by(|a: &(i32, i32),b: &(i32, i32)| a.1.cmp(&b.1));
            sum += numbers[0].0*10 + numbers[numbers.len()-1].0;
        }
    }
    return sum
}

// returns numbers and their positions within strings
// e.g "onetwo12onetwo" will return [(1,6),(2,7)]
fn find_number_digits(code: &String) -> Vec<(i32,i32)> {
    let matches: Vec<(i32,i32)> = code
            .match_indices(char::is_numeric)
            .map(|(index,num)| (num.parse::<i32>().unwrap(),index.try_into().unwrap()))
            .collect();
    return matches
}

// returns the number representations and positions of numbers in a string
// e.g "onetwo" will return [(1,0),(2,3)]
fn find_number_strings(code: &String) -> Vec<(i32,i32)> {
    let mut matches: Vec<(i32,i32)> = Vec::new();
    let numbers_str: [&str; 10] = ["zero","one","two","three","four","five","six","seven","eight","nine"];
    for i in 0..=9 {
        let mut ms: Vec<(i32,i32)> = code
            .match_indices(numbers_str[i])
            .map(|(index,_str)| (i.try_into().unwrap(),index.try_into().unwrap()))
            .collect();
        matches.append(&mut ms)
    }
    return matches
}




