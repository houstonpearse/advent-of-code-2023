use std::{fs::read_to_string, iter::zip};

pub fn solution1() -> i32 {
    let input: Vec<String> = read_to_string("./inputs/input6.txt").unwrap().lines().map(String::from).collect();
    let times: Vec<i32> = input[0].split_whitespace().filter(|x| x.parse::<i32>().is_ok()).map(|x| x.parse::<i32>().unwrap()).collect();
    let records: Vec<i32> = input[1].split_whitespace().filter(|x| x.parse::<i32>().is_ok()).map(|x| x.parse::<i32>().unwrap()).collect();
    let data: Vec<(i32,i32)> = zip(times,records).collect();
    let result: i32 = data.iter()
        .map(|(time, record)| 
                (0..*time)
                .filter(|sec| sec * (time - sec) > *record)
                .count() as i32
            )
        .product();

    return result
}

pub fn solution2() -> i32 {
    return 0
}