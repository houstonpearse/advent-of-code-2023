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

pub fn solution2() -> i64 {
    let input: Vec<String> = read_to_string("./inputs/input6.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let time: i64 = input[0]
        .split_whitespace()
        .filter(|x| x.parse::<i32>().is_ok())
        .fold(String::new(),|acc, e| acc + e)
        .parse::<i64>()
        .unwrap();
    let record: i64 = input[1]
        .split_whitespace()
        .filter(|x| x.parse::<i32>().is_ok())
        .fold(String::new(),|acc, e| acc + e)
        .parse::<i64>()
        .unwrap();

    // sec * (time - sec) > record
    // - sec^2 + time*sec - record > 0
    // sec^2 - time*sec + record < 0
    // time/2 - root(time^2-4*record)/2 < sec <  = time/2 + root((time^2-4*record)/2
    let discriminant = f64::sqrt((((time.pow(2))-4*record)/4) as f64);
    let lower = (time/2) as f64 - discriminant;
    let upper = (time/2) as f64 + discriminant;
    return upper as i64 + 1 - (lower as i64 + 1);
}