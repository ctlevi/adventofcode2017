use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

pub fn solution() {
    let mut f = File::open("input/day4.txt").expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect(
        "something went wrong reading the file",
    );

    println!("Day 4");
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
    println!("-----------------");
}

fn part1(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let mut words = HashSet::new();
        let mut duplicate_found = false;
        for word in line.split(' ') {
            if words.contains(word) {
                duplicate_found = true;
                break;
            }
            words.insert(word);
        }

        if !duplicate_found {
            count += 1;
        }
    }
    return count;
}

fn part2(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let mut words = HashSet::new();
        let mut duplicate_found = false;
        for word in line.split(' ') {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            let sorted_word: String = chars.into_iter().collect();

            if words.contains(&sorted_word) {
                duplicate_found = true;
                break;
            }
            words.insert(sorted_word);
        }

        if !duplicate_found {
            count += 1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(1, part1("aa bb cc dd ee"));
    }

    #[test]
    fn example2() {
        assert_eq!(0, part1("aa bb cc dd ee aa"));
    }

    #[test]
    fn example3() {
        assert_eq!(1, part1("aa bb cc dd aaa"));
    }

    #[test]
    fn example4() {
        assert_eq!(3, part1("aa bb\naa bb\naa cc\naa aa"));
    }

    #[test]
    fn solution() {
        let mut f = File::open("input/day4.txt").expect("file not found");
        let mut input = String::new();
        f.read_to_string(&mut input).expect(
            "something went wrong reading the file",
        );
        assert_eq!(451, part1(&input));
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(1, part2("abcde fghij"));
    }

    #[test]
    fn example2() {
        assert_eq!(0, part2("abcde xyz ecdab"));
    }

    #[test]
    fn example3() {
        assert_eq!(
            2,
            part2(
                "a ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio",
            )
        );
    }

    #[test]
    fn example4() {
        assert_eq!(1, part2("ad bc"));
    }

    #[test]
    fn solution() {
        let mut f = File::open("input/day4.txt").expect("file not found");
        let mut input = String::new();
        f.read_to_string(&mut input).expect(
            "something went wrong reading the file",
        );
        assert_eq!(223, part2(&input));
    }
}
