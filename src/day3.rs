use std::fs::read_to_string;


pub fn solution1() -> i32 {
    let lines = read_lines("./inputs/input3.txt");
    
    let grid = Engine {
        grid: lines.iter().map(|s: &String| s.chars().collect()).collect(), 
        width:lines[0].len().try_into().unwrap(), 
        height: lines.len().try_into().unwrap()
    };
    let mut sum = 0;
    for number in grid.get_engine_numbers().iter() {
        if grid.is_part(number){
            sum += grid.get_part_value(number);
        }
    }
    return sum;
}

pub fn solution2() -> i32 {
    return 0;
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}

#[derive(Debug)]
struct Engine {
    grid: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl Engine {
    fn get_engine_numbers(&self) -> Vec<Vec<(i32,i32)>> {
        let mut numbers: Vec<Vec<(i32,i32)>> = Vec::new();
        for (y,grid_line) in self.grid.iter().enumerate() {
            let mut number: Vec<(i32,i32)> = Vec::new();
            for (x, character) in grid_line.iter().enumerate() {
                if character.is_ascii_digit()  {
                    number.push((x.try_into().unwrap(),y.try_into().unwrap()));
                } else if number.len()>0 {
                    numbers.push(number);
                    number = Vec::new();
                }
            }
            if number.len()>0 {
                numbers.push(number);
            }
        }
        return numbers;
    }
    fn get_adjacent_cells(&self, position: &(i32,i32)) -> Vec<(i32,i32)> {
        let x: i32 = position.0;
        let y: i32 = position.1;
        let mut cells:Vec<(i32,i32)> = Vec::new();
        cells.push((x-1,y-1));
        cells.push((x-1,y));
        cells.push((x-1,y+1));
        cells.push((x,y-1));
        cells.push((x,y+1));
        cells.push((x+1,y-1));
        cells.push((x+1,y));
        cells.push((x+1,y+1));
        cells.retain(|&(x,y)| x >= 0 && x <= self.width-1 && y>=0 && y<=self.height-1);
        return cells
    }

    fn get_cell_value(&self, position: &(i32,i32)) -> char {
        return self.grid[position.1 as usize][position.0 as usize];
    }

    fn has_adjacent_symbol(&self, position: &(i32,i32)) -> bool {
        self.get_adjacent_cells(position).iter().any(|pos| !self.get_cell_value(pos).is_ascii_alphanumeric() && self.get_cell_value(pos) != '.')
    }

    fn is_part(&self, cells: &Vec<(i32,i32)>) -> bool {
        cells.iter().any(|position| self.has_adjacent_symbol(position))
    }

    fn get_part_value(&self, cells: &Vec<(i32,i32)>) -> i32 {
        let mut sum: i32 = 0;
        for (i,position) in cells.iter().rev().enumerate() {
            sum += (self.get_cell_value(position).to_digit(10).unwrap() as i32) * (10_i32.pow(i as u32));
        }
        return sum;
    }

}

