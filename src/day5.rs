use std::fs::File;
use std::io::prelude::*;

pub fn solution() {
    let mut f = File::open("input/day5.txt").expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect(
        "something went wrong reading the file",
    );

    println!("Day 5");
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    println!("-----------------");
}

fn part1(input: &str) -> u32 {
    let mut instructions: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut steps = 0;
    let mut counter: i32 = 0;
    while counter >= 0 && counter < instructions.len() as i32 {
        let instruction = instructions[counter as usize];
        instructions[counter as usize] += 1;
        counter += instruction;
        steps += 1;
    }
    return steps;
}

fn part2(input: &str) -> u32 {
    let mut instructions: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut steps = 0;
    let mut counter: i32 = 0;
    while counter >= 0 && counter < instructions.len() as i32 {
        let instruction = instructions[counter as usize];
        if instruction >= 3 {
            instructions[counter as usize] -= 1;
        } else {
            instructions[counter as usize] += 1;
        }
        counter += instruction;
        steps += 1;
    }
    return steps;
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(5, part1("0\n3\n0\n1\n-3"));
    }

    #[test]
    fn solution() {
        let mut f = File::open("input/day5.txt").expect("file not found");
        let mut input = String::new();
        f.read_to_string(&mut input).expect(
            "something went wrong reading the file",
        );
        assert_eq!(381680, part1(&input));
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(10, part2("0\n3\n0\n1\n-3"));
    }

    #[test]
    fn solution() {
        let mut f = File::open("input/day5.txt").expect("file not found");
        let mut input = String::new();
        f.read_to_string(&mut input).expect(
            "something went wrong reading the file",
        );
        assert_eq!(29717847, part2(&input));
    }
}
