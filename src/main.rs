mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day7b;
fn main() {
    println!("day1 solution1 = {}",day1::solution1());
    println!("day1 solution2 = {}",day1::solution2());
    println!("day2 solution1 = {}",day2::solution1());
    println!("day2 solution2 = {}",day2::solution2());
    println!("day3 solution1 = {}",day3::solution1());
    println!("day3 solution2 = {}",day3::solution2());
    println!("day4 solution1 = {}",day4::solution1());
    println!("day4 solution2 = {}",day4::solution2());
    println!("day5 solution1 = {}",day5::solution1());
    println!("day5 solution2 = {}",/*day5::solution2()*/"slow 30s (69323688)");
    println!("day6 solution1 = {}",day6::solution1());
    println!("day6 solution2 = {}",day6::solution2());
    println!("day7 solution1 = {}",day7::solution1());
    println!("day7 solution2 = {}",day7b::solution2());
}
